//! Lab 04 — inspect UTXOs and outpoints.

use crate::model::{OutPoint, Utxo};
use crate::rpc::RpcClient;
use crate::LabResult;
use serde_json::Value;

/// Return all UTXOs tracked by the selected wallet.
pub fn list_unspent<C: RpcClient>(client: &C, wallet_name: &str) -> LabResult<Vec<Utxo>> {
    let raw = client.call(Some(wallet_name), "listunspent", &[])?;
    let arr: Vec<Value> = serde_json::from_str(&raw)?;
    let utxos = arr
        .into_iter()
        .map(|v| {
            let address = v
                .get("address")
                .and_then(|a| a.as_str())
                .map(ToOwned::to_owned);
            Ok(Utxo {
                txid: v
                    .get("txid")
                    .and_then(Value::as_str)
                    .ok_or(crate::LabError::MissingField("txid"))?
                    .to_owned(),
                vout: v
                    .get("vout")
                    .and_then(Value::as_u64)
                    .ok_or(crate::LabError::MissingField("vout"))? as u32,
                address,
                script_pub_key: v
                    .get("scriptPubKey")
                    .and_then(Value::as_str)
                    .ok_or(crate::LabError::MissingField("scriptPubKey"))?
                    .to_owned(),
                amount: v
                    .get("amount")
                    .and_then(Value::as_f64)
                    .ok_or(crate::LabError::MissingField("amount"))?,
                confirmations: v
                    .get("confirmations")
                    .and_then(Value::as_u64)
                    .ok_or(crate::LabError::MissingField("confirmations"))?,
                spendable: v.get("spendable").and_then(Value::as_bool).unwrap_or(false),
            })
        })
        .collect::<Result<Vec<Utxo>, crate::LabError>>()?;
    Ok(utxos)
}

/// Select one spendable UTXO, preferring the one with the most confirmations.
pub fn select_spendable_utxo(utxos: &[Utxo]) -> Option<Utxo> {
    utxos
        .iter()
        .filter(|u| u.spendable)
        .max_by_key(|u| u.confirmations)
        .cloned()
}

/// Convert a UTXO into its unique `txid:vout` coordinate.
pub fn outpoint(utxo: &Utxo) -> OutPoint {
    utxo.outpoint()
}

/// Sum only the spendable UTXOs.
pub fn sum_spendable_utxos(utxos: &[Utxo]) -> f64 {
    utxos.iter().filter(|u| u.spendable).map(|u| u.amount).sum()
}
