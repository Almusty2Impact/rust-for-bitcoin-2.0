mod support;

use rfb_labs_week_1::labs::lab05_mempool::{
    get_raw_mempool, get_transaction_status, observe_unconfirmed_payment, send_btc,
};
use rfb_labs_week_1::model::{MempoolObservation, WalletBalances, WalletTransactionStatus};
use support::MockRpc;

#[test]
fn sends_payment_in_sender_wallet_context() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        Some("miner"),
        "sendtoaddress",
        &["bcrt1qreceiver", "1"],
        "payment-txid",
    );

    assert_eq!(
        send_btc(&rpc, "miner", "bcrt1qreceiver", 1.0).unwrap(),
        "payment-txid"
    );
    rpc.assert_done();
}

#[test]
fn reads_local_mempool_txids() {
    let rpc = MockRpc::new();
    rpc.expect_ok(None, "getrawmempool", &[], r#"["payment-txid"]"#);

    assert_eq!(get_raw_mempool(&rpc).unwrap(), vec!["payment-txid"]);
    rpc.assert_done();
}

#[test]
fn reads_wallet_transaction_status() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        Some("miner"),
        "gettransaction",
        &["payment-txid"],
        r#"{"txid":"payment-txid","amount":-1.0,"fee":-0.00001,"confirmations":0}"#,
    );

    assert_eq!(
        get_transaction_status(&rpc, "miner", "payment-txid").unwrap(),
        WalletTransactionStatus {
            txid: "payment-txid".to_owned(),
            confirmations: 0,
            amount: -1.0,
            fee: Some(-0.00001),
            block_hash: None,
        }
    );
    rpc.assert_done();
}

#[test]
fn observes_broadcast_without_confirmation() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        Some("miner"),
        "sendtoaddress",
        &["bcrt1qreceiver", "1"],
        "payment-txid",
    );
    rpc.expect_ok(None, "getrawmempool", &[], r#"["payment-txid"]"#);
    rpc.expect_ok(
        Some("miner"),
        "gettransaction",
        &["payment-txid"],
        r#"{"txid":"payment-txid","amount":-1.0,"fee":-0.00001,"confirmations":0}"#,
    );
    rpc.expect_ok(
        Some("receiver"),
        "getbalances",
        &[],
        r#"{"mine":{"trusted":0.0,"untrusted_pending":1.0,"immature":0.0}}"#,
    );

    assert_eq!(
        observe_unconfirmed_payment(&rpc, "miner", "receiver", "bcrt1qreceiver", 1.0).unwrap(),
        MempoolObservation {
            txid: "payment-txid".to_owned(),
            mempool_contains_tx: true,
            sender_status: WalletTransactionStatus {
                txid: "payment-txid".to_owned(),
                confirmations: 0,
                amount: -1.0,
                fee: Some(-0.00001),
                block_hash: None,
            },
            receiver_balance: WalletBalances {
                trusted: 0.0,
                untrusted_pending: 1.0,
                immature: 0.0,
            },
        }
    );
    rpc.assert_done();
}
