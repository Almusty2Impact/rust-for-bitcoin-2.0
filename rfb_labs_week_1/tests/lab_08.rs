mod support;

use rfb_labs_week_1::labs::lab08_security::{
    build_security_report, get_block_header, get_confirmations, mine_additional_blocks,
};
use rfb_labs_week_1::model::{BlockHeaderEvidence, SecurityReport};
use support::MockRpc;

fn header() -> BlockHeaderEvidence {
    BlockHeaderEvidence {
        hash: "block-hash".to_owned(),
        height: 102,
        previous_block_hash: Some("previous-hash".to_owned()),
        merkle_root: "merkle-root".to_owned(),
        nonce: 7,
        difficulty: 0.000_000_01,
        bits: "207fffff".to_owned(),
        confirmations: 1,
        chainwork: "00000000000000ce".to_owned(),
    }
}

const HEADER_JSON: &str = r#"{"hash":"block-hash","height":102,"previousblockhash":"previous-hash","merkleroot":"merkle-root","nonce":7,"difficulty":0.00000001,"bits":"207fffff","confirmations":1,"chainwork":"00000000000000ce"}"#;

#[test]
fn decodes_proof_linked_block_header() {
    let rpc = MockRpc::new();
    rpc.expect_ok(None, "getblockheader", &["block-hash"], HEADER_JSON);

    assert_eq!(get_block_header(&rpc, "block-hash").unwrap(), header());
    rpc.assert_done();
}

#[test]
fn mines_requested_confirmation_depth() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        None,
        "generatetoaddress",
        &["5", "bcrt1qminer"],
        r#"["b1","b2","b3","b4","b5"]"#,
    );

    assert_eq!(
        mine_additional_blocks(&rpc, "bcrt1qminer", 5)
            .unwrap()
            .len(),
        5
    );
    rpc.assert_done();
}

#[test]
fn reads_wallet_confirmation_depth() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        Some("receiver"),
        "gettransaction",
        &["payment-txid"],
        r#"{"confirmations":6}"#,
    );

    assert_eq!(
        get_confirmations(&rpc, "receiver", "payment-txid").unwrap(),
        6
    );
    rpc.assert_done();
}

#[test]
fn proves_one_confirmation_becomes_six() {
    let rpc = MockRpc::new();
    rpc.expect_ok(None, "getblockheader", &["block-hash"], HEADER_JSON);
    rpc.expect_ok(
        Some("receiver"),
        "gettransaction",
        &["payment-txid"],
        r#"{"confirmations":1}"#,
    );
    rpc.expect_ok(
        None,
        "generatetoaddress",
        &["5", "bcrt1qminer"],
        r#"["b1","b2","b3","b4","b5"]"#,
    );
    rpc.expect_ok(
        Some("receiver"),
        "gettransaction",
        &["payment-txid"],
        r#"{"confirmations":6}"#,
    );

    assert_eq!(
        build_security_report(
            &rpc,
            "receiver",
            "payment-txid",
            "block-hash",
            "bcrt1qminer"
        )
        .unwrap(),
        SecurityReport {
            header: header(),
            confirmations_before: 1,
            confirmations_after: 6,
        }
    );
    rpc.assert_done();
}
