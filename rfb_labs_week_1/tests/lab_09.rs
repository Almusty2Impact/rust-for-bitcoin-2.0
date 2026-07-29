mod support;

use rfb_labs_week_1::labs::lab09_coin_selection::{
    audit_multi_utxo_spend, confirmed_utxos_for_address, create_three_funding_transactions,
    send_combined_payment,
};
use rfb_labs_week_1::model::{DecodedOutput, MultiUtxoAudit, OutPoint, PaymentAndChange, Utxo};
use support::MockRpc;

fn funding_utxos() -> Vec<Utxo> {
    (0..3)
        .map(|index| Utxo {
            txid: format!("funding-{index}"),
            vout: 0,
            address: Some("bcrt1qalice".to_owned()),
            script_pub_key: format!("0014{index}"),
            amount: 0.4,
            confirmations: 1,
            spendable: true,
        })
        .collect()
}

#[test]
fn creates_three_separate_funding_transactions() {
    let rpc = MockRpc::new();
    for index in 0..3 {
        rpc.expect_ok(
            Some("miner"),
            "sendtoaddress",
            &["bcrt1qalice", "0.4"],
            &format!("funding-{index}"),
        );
    }

    assert_eq!(
        create_three_funding_transactions(&rpc, "miner", "bcrt1qalice").unwrap(),
        vec!["funding-0", "funding-1", "funding-2"]
    );
    rpc.assert_done();
}

#[test]
fn filters_confirmed_utxos_for_alice_address() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        Some("alice"),
        "listunspent",
        &[],
        r#"[{"txid":"funding-0","vout":0,"address":"bcrt1qalice","scriptPubKey":"00140","amount":0.4,"confirmations":1,"spendable":true},{"txid":"other","vout":1,"address":"bcrt1qother","scriptPubKey":"00149","amount":2.0,"confirmations":1,"spendable":true}]"#,
    );

    assert_eq!(
        confirmed_utxos_for_address(&rpc, "alice", "bcrt1qalice").unwrap(),
        vec![funding_utxos()[0].clone()]
    );
    rpc.assert_done();
}

#[test]
fn sends_one_btc_from_alice() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        Some("alice"),
        "sendtoaddress",
        &["bcrt1qreceiver", "1"],
        "combined-spend",
    );

    assert_eq!(
        send_combined_payment(&rpc, "alice", "bcrt1qreceiver").unwrap(),
        "combined-spend"
    );
    rpc.assert_done();
}

#[test]
fn audits_three_input_spend_payment_change_and_fee() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        Some("alice"),
        "sendtoaddress",
        &["bcrt1qreceiver", "1"],
        "combined-spend",
    );
    rpc.expect_ok(
        None,
        "getrawtransaction",
        &["combined-spend", "2"],
        r#"{"txid":"combined-spend","vsize":209,"vin":[{"txid":"funding-0","vout":0,"prevout":{"value":0.4}},{"txid":"funding-1","vout":0,"prevout":{"value":0.4}},{"txid":"funding-2","vout":0,"prevout":{"value":0.4}}],"vout":[{"value":1.0,"n":0,"scriptPubKey":{"hex":"0014aa","address":"bcrt1qreceiver"}},{"value":0.19999,"n":1,"scriptPubKey":{"hex":"0014bb","address":"bcrt1qalicechange"}}]}"#,
    );

    assert_eq!(
        audit_multi_utxo_spend(&rpc, "alice", "bcrt1qreceiver", &funding_utxos()).unwrap(),
        MultiUtxoAudit {
            funding_outpoints: vec![
                OutPoint {
                    txid: "funding-0".to_owned(),
                    vout: 0,
                },
                OutPoint {
                    txid: "funding-1".to_owned(),
                    vout: 0,
                },
                OutPoint {
                    txid: "funding-2".to_owned(),
                    vout: 0,
                },
            ],
            spend_txid: "combined-spend".to_owned(),
            spend_input_count: 3,
            payment_and_change: PaymentAndChange {
                payment: DecodedOutput {
                    vout: 0,
                    value: 1.0,
                    address: Some("bcrt1qreceiver".to_owned()),
                    script_pub_key_hex: "0014aa".to_owned(),
                },
                change: Some(DecodedOutput {
                    vout: 1,
                    value: 0.19999,
                    address: Some("bcrt1qalicechange".to_owned()),
                    script_pub_key_hex: "0014bb".to_owned(),
                }),
            },
            fee: 0.00001,
        }
    );
    rpc.assert_done();
}
