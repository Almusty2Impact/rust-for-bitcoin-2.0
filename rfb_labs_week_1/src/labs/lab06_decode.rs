//! Lab 06 — decode a transaction and prove value conservation.

use crate::model::{DecodedInput, DecodedOutput, DecodedTransaction, OutPoint, PaymentAndChange};
use crate::rpc::RpcClient;
use crate::LabResult;
use serde_json::Value;

/// Decode a transaction with enough verbosity to include every spent output's value.
pub fn decode_verbose_transaction<C: RpcClient>(
    client: &C,
    txid: &str,
) -> LabResult<DecodedTransaction> {
    let raw = client.call(
        None,
        "getrawtransaction",
        &[txid.to_owned(), "2".to_owned()],
    )?;
    let v: Value = serde_json::from_str(&raw)?;

    let txid = v
        .get("txid")
        .and_then(Value::as_str)
        .ok_or(crate::LabError::MissingField("txid"))?
        .to_owned();

    let vsize = v
        .get("vsize")
        .and_then(Value::as_u64)
        .ok_or(crate::LabError::MissingField("vsize"))?;

    let inputs: Vec<DecodedInput> = v
        .get("vin")
        .and_then(Value::as_array)
        .ok_or(crate::LabError::MissingField("vin"))?
        .iter()
        .map(|vin| {
            let prev_txid = vin
                .get("txid")
                .and_then(Value::as_str)
                .ok_or(crate::LabError::MissingField("vin.txid"))?
                .to_owned();
            let prev_vout =
                vin.get("vout")
                    .and_then(Value::as_u64)
                    .ok_or(crate::LabError::MissingField("vin.vout"))? as u32;
            let prev_value = vin
                .get("prevout")
                .and_then(|p| p.get("value"))
                .and_then(Value::as_f64)
                .ok_or(crate::LabError::MissingField("vin.prevout.value"))?;
            Ok(DecodedInput {
                previous_output: OutPoint {
                    txid: prev_txid,
                    vout: prev_vout,
                },
                previous_value: prev_value,
            })
        })
        .collect::<Result<Vec<DecodedInput>, crate::LabError>>()?;

    let outputs: Vec<DecodedOutput> = v
        .get("vout")
        .and_then(Value::as_array)
        .ok_or(crate::LabError::MissingField("vout"))?
        .iter()
        .map(|vout| {
            let n = vout
                .get("n")
                .and_then(Value::as_u64)
                .ok_or(crate::LabError::MissingField("vout.n"))? as u32;
            let value = vout
                .get("value")
                .and_then(Value::as_f64)
                .ok_or(crate::LabError::MissingField("vout.value"))?;
            let spk = vout
                .get("scriptPubKey")
                .ok_or(crate::LabError::MissingField("scriptPubKey"))?;
            let script_pub_key_hex = spk
                .get("hex")
                .and_then(Value::as_str)
                .ok_or(crate::LabError::MissingField("scriptPubKey.hex"))?
                .to_owned();
            let address = spk
                .get("address")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned);
            Ok(DecodedOutput {
                vout: n,
                value,
                address,
                script_pub_key_hex,
            })
        })
        .collect::<Result<Vec<DecodedOutput>, crate::LabError>>()?;

    Ok(DecodedTransaction {
        txid,
        inputs,
        outputs,
        vsize,
    })
}

/// Return every previous output consumed by the transaction.
pub fn input_outpoints(transaction: &DecodedTransaction) -> Vec<OutPoint> {
    transaction
        .inputs
        .iter()
        .map(|inp| inp.previous_output.clone())
        .collect()
}

/// Identify the receiver payment and optional change output.
pub fn identify_payment_and_change(
    transaction: &DecodedTransaction,
    receiver_address: &str,
) -> LabResult<PaymentAndChange> {
    let mut payment = None;
    let mut change = None;

    for output in &transaction.outputs {
        if let Some(ref addr) = output.address {
            if addr == receiver_address {
                payment = Some(output.clone());
                continue;
            }
        }
        // Treat the remaining non-OP_RETURN output as change.
        if output.script_pub_key_hex.starts_with("6a") {
            continue; // OP_RETURN
        }
        change = Some(output.clone());
    }

    let payment = payment.ok_or(crate::LabError::Parse(
        "receiver payment output not found".to_owned(),
    ))?;

    Ok(PaymentAndChange { payment, change })
}

/// Calculate `sum(inputs) - sum(outputs)`.
pub fn calculate_fee(transaction: &DecodedTransaction) -> LabResult<f64> {
    let input_sum: f64 = transaction.inputs.iter().map(|i| i.previous_value).sum();
    let output_sum: f64 = transaction.outputs.iter().map(|o| o.value).sum();
    let fee = input_sum - output_sum;
    if fee < 0.0 {
        return Err(crate::LabError::Parse(
            "negative fee: outputs exceed inputs".to_owned(),
        ));
    }
    // Round to satoshi precision (8 decimal places) to avoid floating-point drift.
    let fee = (fee * 100_000_000.0).round() / 100_000_000.0;
    Ok(fee)
}
