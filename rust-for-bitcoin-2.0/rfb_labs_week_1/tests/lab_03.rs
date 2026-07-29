mod support;

use rfb_labs_week_1::labs::lab03_maturity::{
    attempt_payment, demonstrate_coinbase_maturity, get_balances, mine_blocks,
};
use rfb_labs_week_1::model::{CoinbaseMaturityReport, WalletBalances};
use rfb_labs_week_1::LabError;
use support::MockRpc;

#[test]
fn mines_requested_number_of_blocks() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        None,
        "generatetoaddress",
        &["2", "bcrt1qminer"],
        r#"["block01","block02"]"#,
    );

    assert_eq!(
        mine_blocks(&rpc, "bcrt1qminer", 2).unwrap(),
        vec!["block01", "block02"]
    );
    rpc.assert_done();
}

#[test]
fn reads_nested_wallet_balances() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        Some("miner"),
        "getbalances",
        &[],
        r#"{"mine":{"trusted":50.0,"untrusted_pending":0.0,"immature":5000.0}}"#,
    );

    assert_eq!(
        get_balances(&rpc, "miner").unwrap(),
        WalletBalances {
            trusted: 50.0,
            untrusted_pending: 0.0,
            immature: 5000.0,
        }
    );
    rpc.assert_done();
}

#[test]
fn preserves_insufficient_funds_error() {
    let rpc = MockRpc::new();
    rpc.expect_rpc_error(
        Some("miner"),
        "sendtoaddress",
        &["bcrt1qreceiver", "1"],
        "Insufficient funds",
    );

    assert_eq!(
        attempt_payment(&rpc, "miner", "bcrt1qreceiver", 1.0),
        Err(LabError::Rpc("Insufficient funds".to_owned()))
    );
    rpc.assert_done();
}

#[test]
fn demonstrates_first_coinbase_becoming_spendable_at_height_101() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        None,
        "generatetoaddress",
        &["1", "bcrt1qminer"],
        r#"["block01"]"#,
    );
    rpc.expect_ok(None, "getblockcount", &[], "1");
    rpc.expect_ok(
        Some("miner"),
        "getbalances",
        &[],
        r#"{"mine":{"trusted":0.0,"untrusted_pending":0.0,"immature":50.0}}"#,
    );
    rpc.expect_rpc_error(
        Some("miner"),
        "sendtoaddress",
        &["bcrt1qreceiver", "1"],
        "Insufficient funds",
    );
    rpc.expect_ok(
        None,
        "generatetoaddress",
        &["100", "bcrt1qminer"],
        r#"["block02","block101"]"#,
    );
    rpc.expect_ok(None, "getblockcount", &[], "101");
    rpc.expect_ok(
        Some("miner"),
        "getbalances",
        &[],
        r#"{"mine":{"trusted":50.0,"untrusted_pending":0.0,"immature":5000.0}}"#,
    );

    assert_eq!(
        demonstrate_coinbase_maturity(&rpc, "miner", "bcrt1qminer", "bcrt1qreceiver").unwrap(),
        CoinbaseMaturityReport {
            height_after_first_block: 1,
            balance_after_first_block: WalletBalances {
                trusted: 0.0,
                untrusted_pending: 0.0,
                immature: 50.0,
            },
            premature_spend_error: "Insufficient funds".to_owned(),
            final_height: 101,
            final_balance: WalletBalances {
                trusted: 50.0,
                untrusted_pending: 0.0,
                immature: 5000.0,
            },
        }
    );
    rpc.assert_done();
}
