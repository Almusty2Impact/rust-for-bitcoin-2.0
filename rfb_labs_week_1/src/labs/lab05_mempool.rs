//! Lab 05 — broadcast a transaction and observe the mempool.

use crate::model::{MempoolObservation, WalletBalances, WalletTransactionStatus};
use crate::rpc::RpcClient;
use crate::LabResult;
use serde_json::Value;

/// Send bitcoin from one wallet and return the TXID.
pub fn send_btc<C: RpcClient>(
    client: &C,
    from_wallet: &str,
    destination: &str,
    amount_btc: f64,
) -> LabResult<String> {
    let raw = client.call(
        Some(from_wallet),
        "sendtoaddress",
        &[destination.to_owned(), amount_btc.to_string()],
    )?;
    Ok(raw)
}

/// Return the node's local mempool as a list of TXIDs.
pub fn get_raw_mempool<C: RpcClient>(client: &C) -> LabResult<Vec<String>> {
    let raw = client.call(None, "getrawmempool", &[])?;
    let txids: Vec<String> = serde_json::from_str(&raw)?;
    Ok(txids)
}

/// Return the selected wallet's view of one transaction.
pub fn get_transaction_status<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
) -> LabResult<WalletTransactionStatus> {
    let raw = client.call(Some(wallet_name), "gettransaction", &[txid.to_owned()])?;
    let v: Value = serde_json::from_str(&raw)?;

    let fee = v.get("fee").and_then(Value::as_f64);
    let block_hash = v
        .get("blockhash")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);

    Ok(WalletTransactionStatus {
        txid: v
            .get("txid")
            .and_then(Value::as_str)
            .ok_or(crate::LabError::MissingField("txid"))?
            .to_owned(),
        confirmations: v
            .get("confirmations")
            .and_then(Value::as_i64)
            .ok_or(crate::LabError::MissingField("confirmations"))?,
        amount: v
            .get("amount")
            .and_then(Value::as_f64)
            .ok_or(crate::LabError::MissingField("amount"))?,
        fee,
        block_hash,
    })
}

/// Send a payment without mining and capture its mempool and receiver-wallet state.
pub fn observe_unconfirmed_payment<C: RpcClient>(
    client: &C,
    sender_wallet: &str,
    receiver_wallet: &str,
    receiver_address: &str,
    amount_btc: f64,
) -> LabResult<MempoolObservation> {
    // Send the payment.
    let txid = send_btc(client, sender_wallet, receiver_address, amount_btc)?;

    // Check mempool.
    let mempool = get_raw_mempool(client)?;
    let mempool_contains_tx = mempool.contains(&txid);

    // Sender status.
    let sender_status = get_transaction_status(client, sender_wallet, &txid)?;

    // Receiver balances.
    let balances_raw = client.call(Some(receiver_wallet), "getbalances", &[])?;
    let balances_v: Value = serde_json::from_str(&balances_raw)?;
    let mine = balances_v
        .get("mine")
        .ok_or(crate::LabError::MissingField("mine"))?;
    let receiver_balance = WalletBalances {
        trusted: mine.get("trusted").and_then(Value::as_f64).unwrap_or(0.0),
        untrusted_pending: mine
            .get("untrusted_pending")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        immature: mine.get("immature").and_then(Value::as_f64).unwrap_or(0.0),
    };

    Ok(MempoolObservation {
        txid,
        mempool_contains_tx,
        sender_status,
        receiver_balance,
    })
}
