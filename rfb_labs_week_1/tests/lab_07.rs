mod support;

use rfb_labs_week_1::labs::lab07_confirm::{
    confirm_and_locate_transaction, mempool_is_empty, mine_one_block, transaction_confirmations,
};
use rfb_labs_week_1::model::ConfirmationReport;
use support::MockRpc;

#[test]
fn mines_exactly_one_block() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        None,
        "generatetoaddress",
        &["1", "bcrt1qminer"],
        r#"["block-hash"]"#,
    );

    assert_eq!(mine_one_block(&rpc, "bcrt1qminer").unwrap(), "block-hash");
    rpc.assert_done();
}

#[test]
fn detects_empty_mempool() {
    let rpc = MockRpc::new();
    rpc.expect_ok(None, "getrawmempool", &[], "[]");

    assert!(mempool_is_empty(&rpc).unwrap());
    rpc.assert_done();
}

#[test]
fn reads_confirmation_count() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        Some("receiver"),
        "gettransaction",
        &["payment-txid"],
        r#"{"txid":"payment-txid","confirmations":1,"blockhash":"block-hash"}"#,
    );

    assert_eq!(
        transaction_confirmations(&rpc, "receiver", "payment-txid").unwrap(),
        1
    );
    rpc.assert_done();
}

#[test]
fn proves_transaction_is_inside_confirming_block() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        None,
        "generatetoaddress",
        &["1", "bcrt1qminer"],
        r#"["block-hash"]"#,
    );
    rpc.expect_ok(None, "getrawmempool", &[], "[]");
    rpc.expect_ok(
        Some("receiver"),
        "gettransaction",
        &["payment-txid"],
        r#"{"txid":"payment-txid","confirmations":1,"blockhash":"block-hash"}"#,
    );
    rpc.expect_ok(
        None,
        "getblock",
        &["block-hash", "1"],
        r#"{"hash":"block-hash","tx":["coinbase-txid","payment-txid"]}"#,
    );

    assert_eq!(
        confirm_and_locate_transaction(&rpc, "receiver", "payment-txid", "bcrt1qminer").unwrap(),
        ConfirmationReport {
            txid: "payment-txid".to_owned(),
            block_hash: "block-hash".to_owned(),
            confirmations: 1,
            mempool_is_empty: true,
            transaction_is_in_block: true,
        }
    );
    rpc.assert_done();
}
