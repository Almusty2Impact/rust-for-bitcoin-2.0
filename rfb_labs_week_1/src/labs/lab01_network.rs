//! Lab 01 — build and verify a regtest network.

use crate::model::NetworkSnapshot;
use crate::rpc::{required_string, required_u64, RpcClient};
use crate::LabResult;
use serde_json::Value;

/// Return the active chain reported by `getblockchaininfo`.
pub fn get_chain<C: RpcClient>(client: &C) -> LabResult<String> {
    let raw = client.call(None, "getblockchaininfo", &[])?;
    let value: Value = serde_json::from_str(&raw)?;
    Ok(required_string(&value, "chain")?)
}

/// Return the current height using the node-wide `getblockcount` RPC.
pub fn get_block_height<C: RpcClient>(client: &C) -> LabResult<u64> {
    let raw = client.call(None, "getblockcount", &[])?;
    let value = crate::rpc::parse_cli_value(&raw)?;
    value
        .as_u64()
        .ok_or_else(|| crate::LabError::Parse("expected numeric block height".to_owned()))
}

/// Return the node's current best-block hash.
pub fn get_best_block_hash<C: RpcClient>(client: &C) -> LabResult<String> {
    let raw = client.call(None, "getbestblockhash", &[])?;
    Ok(raw)
}

/// Collect the chain, height, and best-block hash into one snapshot.
pub fn inspect_network<C: RpcClient>(client: &C) -> LabResult<NetworkSnapshot> {
    let chain = get_chain(client)?;
    if chain != "regtest" {
        return Err(crate::LabError::Rpc(format!(
            "expected regtest chain, got {chain}"
        )));
    }
    let block_height = get_block_height(client)?;
    let best_block_hash = get_best_block_hash(client)?;
    Ok(NetworkSnapshot {
        chain,
        block_height,
        best_block_hash,
    })
}
