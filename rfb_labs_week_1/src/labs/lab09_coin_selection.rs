//! Lab 09 — force and audit multi-UTXO coin selection.

use crate::model::{MultiUtxoAudit, Utxo};
use crate::rpc::RpcClient;
use crate::LabResult;

/// Send three separate 0.4 BTC funding transactions and return their TXIDs.
pub fn create_three_funding_transactions<C: RpcClient>(
    client: &C,
    miner_wallet: &str,
    alice_address: &str,
) -> LabResult<Vec<String>> {
    // TODO: call sendtoaddress three times, each for 0.4 BTC.
    todo!("Lab 09: create three separate funding transactions")
}

/// Return confirmed UTXOs belonging to the supplied address.
pub fn confirmed_utxos_for_address<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    address: &str,
) -> LabResult<Vec<Utxo>> {
    // TODO: call listunspent and retain confirmed outputs for this address.
    todo!("Lab 09: locate Alice's confirmed UTXOs")
}

/// Send 1 BTC from Alice and return the new TXID.
pub fn send_combined_payment<C: RpcClient>(
    client: &C,
    alice_wallet: &str,
    receiver_address: &str,
) -> LabResult<String> {
    // TODO: call sendtoaddress for 1 BTC.
    todo!("Lab 09: create a spend requiring multiple inputs")
}

/// Decode Alice's spend and prove that multiple funding UTXOs were combined.
pub fn audit_multi_utxo_spend<C: RpcClient>(
    client: &C,
    alice_wallet: &str,
    receiver_address: &str,
    funding_utxos: &[Utxo],
) -> LabResult<MultiUtxoAudit> {
    // TODO:
    // 1. Send the 1 BTC payment.
    // 2. Reuse Lab 06 to decode it.
    // 3. Identify payment and change.
    // 4. Calculate fee and input count.
    // 5. Record the funding outpoints.
    todo!("Lab 09: audit multi-UTXO coin selection")
}
