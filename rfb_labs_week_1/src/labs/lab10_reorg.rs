//! Lab 10 — observe competing branches and most-work convergence.

use crate::model::{ChainTip, ForkSnapshot, ReorgReport};
use crate::rpc::RpcClient;
use crate::LabResult;
use serde_json::Value;

/// Read height, best-block hash, and accumulated chainwork from one node.
pub fn get_chain_tip<C: RpcClient>(client: &C) -> LabResult<ChainTip> {
    let raw = client.call(None, "getblockchaininfo", &[])?;
    let v: Value = serde_json::from_str(&raw)?;

    Ok(ChainTip {
        height: v
            .get("blocks")
            .and_then(Value::as_u64)
            .ok_or(crate::LabError::MissingField("blocks"))?,
        best_block_hash: v
            .get("bestblockhash")
            .and_then(Value::as_str)
            .ok_or(crate::LabError::MissingField("bestblockhash"))?
            .to_owned(),
        chainwork: v
            .get("chainwork")
            .and_then(Value::as_str)
            .ok_or(crate::LabError::MissingField("chainwork"))?
            .to_owned(),
    })
}

/// Disconnect a peer by its address.
pub fn disconnect_peer<C: RpcClient>(client: &C, peer_address: &str) -> LabResult<()> {
    client.call(None, "disconnectnode", &[peer_address.to_owned()])?;
    Ok(())
}

/// Reconnect a peer for a one-time synchronization attempt.
pub fn reconnect_peer<C: RpcClient>(client: &C, peer_address: &str) -> LabResult<()> {
    client.call(
        None,
        "addnode",
        &[peer_address.to_owned(), "onetry".to_owned()],
    )?;
    Ok(())
}

/// Compare the private competing tips with the final synchronized tips.
pub fn build_reorg_report(
    common_tip_before_split: &str,
    competing_tips: ForkSnapshot,
    final_tips: ForkSnapshot,
) -> ReorgReport {
    let converged = final_tips.node_a == final_tips.node_b;

    ReorgReport {
        common_tip_before_split: common_tip_before_split.to_owned(),
        competing_tips,
        final_tips,
        converged,
    }
}
