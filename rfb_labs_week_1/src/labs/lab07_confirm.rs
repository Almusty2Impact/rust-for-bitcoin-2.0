//! Lab 07 — confirm a transaction and prove block membership.

use crate::model::ConfirmationReport;
use crate::rpc::RpcClient;
use crate::LabResult;

/// Mine exactly one block and return its hash.
pub fn mine_one_block<C: RpcClient>(client: &C, miner_address: &str) -> LabResult<String> {
    // TODO: call generatetoaddress with a count of one.
    todo!("Lab 07: mine one block")
}

/// Return true only when this node's mempool contains no transactions.
pub fn mempool_is_empty<C: RpcClient>(client: &C) -> LabResult<bool> {
    // TODO: inspect getrawmempool.
    todo!("Lab 07: check whether the mempool is empty")
}

/// Return a transaction's confirmation count in the selected wallet.
pub fn transaction_confirmations<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
) -> LabResult<i64> {
    // TODO: call gettransaction and return confirmations.
    todo!("Lab 07: read transaction confirmations")
}

/// Mine, locate the transaction's block, and prove that the block contains the TXID.
pub fn confirm_and_locate_transaction<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
    miner_address: &str,
) -> LabResult<ConfirmationReport> {
    // TODO:
    // 1. Mine one block.
    // 2. Check the mempool.
    // 3. Read gettransaction for blockhash and confirmations.
    // 4. Read getblock and verify that its `tx` array contains txid.
    todo!("Lab 07: prove confirmation and block membership")
}
