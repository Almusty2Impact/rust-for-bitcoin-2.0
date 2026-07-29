//! Lab 05 — broadcast a transaction and observe the mempool.

use crate::model::{MempoolObservation, WalletBalances, WalletTransactionStatus};
use crate::rpc::RpcClient;
use crate::LabResult;

/// Send bitcoin from one wallet and return the TXID.
pub fn send_btc<C: RpcClient>(
    client: &C,
    from_wallet: &str,
    destination: &str,
    amount_btc: f64,
) -> LabResult<String> {
    // TODO: call sendtoaddress in the sender's wallet context.
    todo!("Lab 05: send bitcoin")
}

/// Return the node's local mempool as a list of TXIDs.
pub fn get_raw_mempool<C: RpcClient>(client: &C) -> LabResult<Vec<String>> {
    // TODO: call getrawmempool and decode its array.
    todo!("Lab 05: inspect the local mempool")
}

/// Return the selected wallet's view of one transaction.
pub fn get_transaction_status<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
) -> LabResult<WalletTransactionStatus> {
    // TODO: call gettransaction and decode txid, amount, fee, confirmations, and blockhash.
    todo!("Lab 05: inspect wallet transaction status")
}

/// Send a payment without mining and capture its mempool and receiver-wallet state.
pub fn observe_unconfirmed_payment<C: RpcClient>(
    client: &C,
    sender_wallet: &str,
    receiver_wallet: &str,
    receiver_address: &str,
    amount_btc: f64,
) -> LabResult<MempoolObservation> {
    // TODO: send, inspect getrawmempool, inspect sender status, and read receiver balances.
    todo!("Lab 05: prove a payment is broadcast but unconfirmed")
}
