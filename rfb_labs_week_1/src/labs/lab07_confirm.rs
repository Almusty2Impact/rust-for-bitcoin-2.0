//! Lab 07 — confirm a transaction and prove block membership.

use crate::model::ConfirmationReport;
use crate::rpc::RpcClient;
use crate::LabResult;
use serde_json::Value;

/// Mine exactly one block and return its hash.
pub fn mine_one_block<C: RpcClient>(client: &C, miner_address: &str) -> LabResult<String> {
    let raw = client.call(
        None,
        "generatetoaddress",
        &["1".to_owned(), miner_address.to_owned()],
    )?;
    let hashes: Vec<String> = serde_json::from_str(&raw)?;
    hashes
        .into_iter()
        .next()
        .ok_or_else(|| crate::LabError::Parse("no block hash returned".to_owned()))
}

/// Return true only when this node's mempool contains no transactions.
pub fn mempool_is_empty<C: RpcClient>(client: &C) -> LabResult<bool> {
    let raw = client.call(None, "getrawmempool", &[])?;
    let txids: Vec<String> = serde_json::from_str(&raw)?;
    Ok(txids.is_empty())
}

/// Return a transaction's confirmation count in the selected wallet.
pub fn transaction_confirmations<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
) -> LabResult<i64> {
    let raw = client.call(Some(wallet_name), "gettransaction", &[txid.to_owned()])?;
    let v: Value = serde_json::from_str(&raw)?;
    v.get("confirmations")
        .and_then(Value::as_i64)
        .ok_or(crate::LabError::MissingField("confirmations"))
}

/// Mine, locate the transaction's block, and prove that the block contains the TXID.
pub fn confirm_and_locate_transaction<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
    miner_address: &str,
) -> LabResult<ConfirmationReport> {
    // 1. Mine one block.
    let block_hash = mine_one_block(client, miner_address)?;

    // 2. Check the mempool.
    let mempool_is_empty = mempool_is_empty(client)?;

    // 3. Read gettransaction for blockhash and confirmations.
    let tx_raw = client.call(Some(wallet_name), "gettransaction", &[txid.to_owned()])?;
    let tx_v: Value = serde_json::from_str(&tx_raw)?;
    let confirmations = tx_v
        .get("confirmations")
        .and_then(Value::as_i64)
        .ok_or(crate::LabError::MissingField("confirmations"))?;
    let tx_block_hash = tx_v
        .get("blockhash")
        .and_then(Value::as_str)
        .ok_or(crate::LabError::MissingField("blockhash"))?
        .to_owned();

    // 4. Read getblock and verify that its `tx` array contains txid.
    let block_raw = client.call(None, "getblock", &[tx_block_hash.clone(), "1".to_owned()])?;
    let block_v: Value = serde_json::from_str(&block_raw)?;
    let block_txs = block_v
        .get("tx")
        .and_then(Value::as_array)
        .ok_or(crate::LabError::MissingField("tx"))?;
    let transaction_is_in_block = block_txs
        .iter()
        .any(|t| t.as_str().map_or(false, |s| s == txid));

    Ok(ConfirmationReport {
        txid: txid.to_owned(),
        block_hash: tx_block_hash,
        confirmations,
        mempool_is_empty,
        transaction_is_in_block,
    })
}
