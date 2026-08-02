//! Lab 08 — inspect proof-linked headers and confirmation depth.

use crate::model::{BlockHeaderEvidence, SecurityReport};
use crate::rpc::RpcClient;
use crate::LabResult;
use serde_json::Value;

/// Decode a block header into the fields used by the lab.
pub fn get_block_header<C: RpcClient>(
    client: &C,
    block_hash: &str,
) -> LabResult<BlockHeaderEvidence> {
    let raw = client.call(None, "getblockheader", &[block_hash.to_owned()])?;
    let v: Value = serde_json::from_str(&raw)?;

    let previous_block_hash = v
        .get("previousblockhash")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);

    Ok(BlockHeaderEvidence {
        hash: v
            .get("hash")
            .and_then(Value::as_str)
            .ok_or(crate::LabError::MissingField("hash"))?
            .to_owned(),
        height: v
            .get("height")
            .and_then(Value::as_u64)
            .ok_or(crate::LabError::MissingField("height"))?,
        previous_block_hash,
        merkle_root: v
            .get("merkleroot")
            .and_then(Value::as_str)
            .ok_or(crate::LabError::MissingField("merkleroot"))?
            .to_owned(),
        nonce: v
            .get("nonce")
            .and_then(Value::as_u64)
            .ok_or(crate::LabError::MissingField("nonce"))?,
        difficulty: v
            .get("difficulty")
            .and_then(Value::as_f64)
            .ok_or(crate::LabError::MissingField("difficulty"))?,
        bits: v
            .get("bits")
            .and_then(Value::as_str)
            .ok_or(crate::LabError::MissingField("bits"))?
            .to_owned(),
        confirmations: v
            .get("confirmations")
            .and_then(Value::as_i64)
            .ok_or(crate::LabError::MissingField("confirmations"))?,
        chainwork: v
            .get("chainwork")
            .and_then(Value::as_str)
            .ok_or(crate::LabError::MissingField("chainwork"))?
            .to_owned(),
    })
}

/// Mine an exact number of additional blocks and return their hashes.
pub fn mine_additional_blocks<C: RpcClient>(
    client: &C,
    miner_address: &str,
    count: u64,
) -> LabResult<Vec<String>> {
    let raw = client.call(
        None,
        "generatetoaddress",
        &[count.to_string(), miner_address.to_owned()],
    )?;
    let hashes: Vec<String> = serde_json::from_str(&raw)?;
    Ok(hashes)
}

/// Read a transaction's confirmation count.
pub fn get_confirmations<C: RpcClient>(
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

/// Record the block header and prove one confirmation becomes six after five blocks.
pub fn build_security_report<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
    block_hash: &str,
    miner_address: &str,
) -> LabResult<SecurityReport> {
    let header = get_block_header(client, block_hash)?;
    let confirmations_before = get_confirmations(client, wallet_name, txid)?;

    mine_additional_blocks(client, miner_address, 5)?;

    let confirmations_after = get_confirmations(client, wallet_name, txid)?;

    Ok(SecurityReport {
        header,
        confirmations_before,
        confirmations_after,
    })
}
