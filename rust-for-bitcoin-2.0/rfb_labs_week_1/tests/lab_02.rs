mod support;

use rfb_labs_week_1::labs::lab02_wallets::{
    address_belongs_to_wallet, create_wallet, get_new_address, list_wallets,
};
use support::MockRpc;

#[test]
fn creates_wallet() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        None,
        "createwallet",
        &["miner"],
        r#"{"name":"miner","warning":""}"#,
    );

    create_wallet(&rpc, "miner").unwrap();
    rpc.assert_done();
}

#[test]
fn lists_loaded_wallets() {
    let rpc = MockRpc::new();
    rpc.expect_ok(None, "listwallets", &[], r#"["miner","receiver"]"#);

    assert_eq!(list_wallets(&rpc).unwrap(), vec!["miner", "receiver"]);
    rpc.assert_done();
}

#[test]
fn generates_labelled_address_in_wallet_context() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        Some("receiver"),
        "getnewaddress",
        &["classmate"],
        "bcrt1qreceiver",
    );

    assert_eq!(
        get_new_address(&rpc, "receiver", "classmate").unwrap(),
        "bcrt1qreceiver"
    );
    rpc.assert_done();
}

#[test]
fn verifies_wallet_owns_address() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        Some("receiver"),
        "getaddressinfo",
        &["bcrt1qreceiver"],
        r#"{"address":"bcrt1qreceiver","ismine":true}"#,
    );

    assert!(address_belongs_to_wallet(&rpc, "receiver", "bcrt1qreceiver").unwrap());
    rpc.assert_done();
}
