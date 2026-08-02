//! Lab 09 — force and audit multi-UTXO coin selection.

use crate::labs::lab06_decode::{
    calculate_fee, decode_verbose_transaction, identify_payment_and_change,
};
use crate::model::{MultiUtxoAudit, OutPoint, PaymentAndChange};
use crate::rpc::RpcClient;
use crate::LabResult;
use serde_json::Value;

/// Send three separate 0.4 BTC funding transactions and return their TXIDs.
pub fn create_three_funding_transactions<C: RpcClient>(
    client: &C,
    miner_wallet: &str,
    alice_address: &str,
) -> LabResult<Vec<String>> {
    let mut txids = Vec::with_capacity(3);
    for _ in 0..3 {
        let raw = client.call(
            Some(miner_wallet),
            "sendtoaddress",
            &[alice_address.to_owned(), "0.4".to_owned()],
        )?;
        txids.push(raw);
    }
    Ok(txids)
}

/// Return confirmed UTXOs belonging to the supplied address.
pub fn confirmed_utxos_for_address<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    address: &str,
) -> LabResult<Vec<crate::model::Utxo>> {
    let raw = client.call(Some(wallet_name), "listunspent", &[])?;
    let arr: Vec<Value> = serde_json::from_str(&raw)?;
    let utxos = arr
        .into_iter()
        .filter_map(|v| {
            let addr = v.get("address").and_then(Value::as_str)?;
            if addr != address {
                return None;
            }
            let confirmations = v.get("confirmations").and_then(Value::as_u64)?;
            if confirmations < 1 {
                return None;
            }
            Some(Ok(crate::model::Utxo {
                txid: v.get("txid").and_then(Value::as_str)?.to_owned(),
                vout: v.get("vout").and_then(Value::as_u64)? as u32,
                address: Some(addr.to_owned()),
                script_pub_key: v.get("scriptPubKey").and_then(Value::as_str)?.to_owned(),
                amount: v.get("amount").and_then(Value::as_f64)?,
                confirmations,
                spendable: v.get("spendable").and_then(Value::as_bool).unwrap_or(false),
            }))
        })
        .collect::<Result<Vec<crate::model::Utxo>, crate::LabError>>()?;
    Ok(utxos)
}

/// Send 1 BTC from Alice and return the new TXID.
pub fn send_combined_payment<C: RpcClient>(
    client: &C,
    alice_wallet: &str,
    receiver_address: &str,
) -> LabResult<String> {
    let raw = client.call(
        Some(alice_wallet),
        "sendtoaddress",
        &[receiver_address.to_owned(), "1".to_owned()],
    )?;
    Ok(raw)
}

/// Decode Alice's spend and prove that multiple funding UTXOs were combined.
pub fn audit_multi_utxo_spend<C: RpcClient>(
    client: &C,
    alice_wallet: &str,
    receiver_address: &str,
    funding_utxos: &[crate::model::Utxo],
) -> LabResult<MultiUtxoAudit> {
    // 1. Send the 1 BTC payment.
    let spend_txid = send_combined_payment(client, alice_wallet, receiver_address)?;

    // 2. Reuse Lab 06 to decode it.
    let decoded = decode_verbose_transaction(client, &spend_txid)?;

    // 3. Identify payment and change.
    let payment_and_change = identify_payment_and_change(&decoded, receiver_address)?;

    // 4. Calculate fee and input count.
    let fee = calculate_fee(&decoded)?;
    let spend_input_count = decoded.inputs.len();

    // 5. Record the funding outpoints.
    let funding_outpoints: Vec<OutPoint> = funding_utxos.iter().map(|u| u.outpoint()).collect();

    Ok(MultiUtxoAudit {
        funding_outpoints,
        spend_txid,
        spend_input_count,
        payment_and_change,
        fee,
    })
}
