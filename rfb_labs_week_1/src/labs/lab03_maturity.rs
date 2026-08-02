//! Lab 03 — demonstrate coinbase maturity.

use crate::model::{CoinbaseMaturityReport, WalletBalances};
use crate::rpc::{required_f64, required_u64, RpcClient};
use crate::LabResult;
use serde_json::Value;

/// Mine `count` blocks to an address and return the generated block hashes.
pub fn mine_blocks<C: RpcClient>(client: &C, address: &str, count: u64) -> LabResult<Vec<String>> {
    let raw = client.call(
        None,
        "generatetoaddress",
        &[count.to_string(), address.to_owned()],
    )?;
    let hashes: Vec<String> = serde_json::from_str(&raw)?;
    Ok(hashes)
}

/// Read the wallet's trusted, untrusted-pending, and immature balances.
pub fn get_balances<C: RpcClient>(client: &C, wallet_name: &str) -> LabResult<WalletBalances> {
    let raw = client.call(Some(wallet_name), "getbalances", &[])?;
    let value: Value = serde_json::from_str(&raw)?;
    let mine = value
        .get("mine")
        .ok_or_else(|| crate::LabError::MissingField("mine"))?;
    Ok(WalletBalances {
        trusted: required_f64(mine, "trusted")?,
        untrusted_pending: required_f64(mine, "untrusted_pending")?,
        immature: required_f64(mine, "immature")?,
    })
}

/// Attempt a wallet payment and return either its TXID or the Bitcoin Core error.
pub fn attempt_payment<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    address: &str,
    amount_btc: f64,
) -> LabResult<String> {
    client.call(
        Some(wallet_name),
        "sendtoaddress",
        &[address.to_owned(), amount_btc.to_string()],
    )
}

/// Mine one block, prove the reward is immature, then mine 100 more blocks.
pub fn demonstrate_coinbase_maturity<C: RpcClient>(
    client: &C,
    miner_wallet: &str,
    miner_address: &str,
    receiver_address: &str,
) -> LabResult<CoinbaseMaturityReport> {
    // 1. Mine one block.
    mine_blocks(client, miner_address, 1)?;

    // 2. Record height and balances.
    let height_raw = client.call(None, "getblockcount", &[])?;
    let height_after_first_block: u64 = serde_json::from_str(&height_raw)?;
    let balance_after_first_block = get_balances(client, miner_wallet)?;

    // 3. Attempt a 1 BTC payment and capture its error text.
    let premature_spend_error = match attempt_payment(client, miner_wallet, receiver_address, 1.0) {
        Ok(_) => "Payment succeeded unexpectedly".to_owned(),
        Err(crate::LabError::Rpc(msg)) => msg,
        Err(e) => e.to_string(),
    };

    // 4. Mine 100 more blocks.
    mine_blocks(client, miner_address, 100)?;

    // 5. Record final height and balances.
    let final_height_raw = client.call(None, "getblockcount", &[])?;
    let final_height: u64 = serde_json::from_str(&final_height_raw)?;
    let final_balance = get_balances(client, miner_wallet)?;

    Ok(CoinbaseMaturityReport {
        height_after_first_block,
        balance_after_first_block,
        premature_spend_error,
        final_height,
        final_balance,
    })
}
