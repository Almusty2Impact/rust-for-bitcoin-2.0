mod support;

use rfb_labs_week_1::labs::lab01_network::{
    get_best_block_hash, get_block_height, get_chain, inspect_network,
};
use rfb_labs_week_1::model::NetworkSnapshot;
use support::MockRpc;

#[test]
fn reads_regtest_chain() {
    let rpc = MockRpc::new();
    rpc.expect_ok(None, "getblockchaininfo", &[], r#"{"chain":"regtest"}"#);

    assert_eq!(get_chain(&rpc).unwrap(), "regtest");
    rpc.assert_done();
}

#[test]
fn reads_block_height() {
    let rpc = MockRpc::new();
    rpc.expect_ok(None, "getblockcount", &[], "101");

    assert_eq!(get_block_height(&rpc).unwrap(), 101);
    rpc.assert_done();
}

#[test]
fn reads_best_block_hash() {
    let rpc = MockRpc::new();
    rpc.expect_ok(None, "getbestblockhash", &[], "0000abcd");

    assert_eq!(get_best_block_hash(&rpc).unwrap(), "0000abcd");
    rpc.assert_done();
}

#[test]
fn builds_verified_network_snapshot() {
    let rpc = MockRpc::new();
    rpc.expect_ok(None, "getblockchaininfo", &[], r#"{"chain":"regtest"}"#);
    rpc.expect_ok(None, "getblockcount", &[], "101");
    rpc.expect_ok(None, "getbestblockhash", &[], "0000abcd");

    assert_eq!(
        inspect_network(&rpc).unwrap(),
        NetworkSnapshot {
            chain: "regtest".to_owned(),
            block_height: 101,
            best_block_hash: "0000abcd".to_owned(),
        }
    );
    rpc.assert_done();
}
