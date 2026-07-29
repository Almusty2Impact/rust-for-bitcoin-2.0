//! Lab 10 — observe competing branches and most-work convergence.

use crate::model::{ChainTip, ForkSnapshot, ReorgReport};
use crate::rpc::RpcClient;
use crate::LabResult;

/// Read height, best-block hash, and accumulated chainwork from one node.
pub fn get_chain_tip<C: RpcClient>(client: &C) -> LabResult<ChainTip> {
    // TODO: call getblockchaininfo and decode blocks, bestblockhash, and chainwork.
    todo!("Lab 10: inspect one node's chain tip")
}

/// Disconnect a peer by its address.
pub fn disconnect_peer<C: RpcClient>(client: &C, peer_address: &str) -> LabResult<()> {
    // TODO: call disconnectnode with the peer address.
    todo!("Lab 10: disconnect competing nodes")
}

/// Reconnect a peer for a one-time synchronization attempt.
pub fn reconnect_peer<C: RpcClient>(client: &C, peer_address: &str) -> LabResult<()> {
    // TODO: call addnode with the address and `onetry`.
    todo!("Lab 10: reconnect competing nodes")
}

/// Compare the private competing tips with the final synchronized tips.
pub fn build_reorg_report(
    common_tip_before_split: &str,
    competing_tips: ForkSnapshot,
    final_tips: ForkSnapshot,
) -> ReorgReport {
    // TODO: nodes converge when their final best hashes and heights match.
    todo!("Lab 10: report most-work-chain convergence")
}
