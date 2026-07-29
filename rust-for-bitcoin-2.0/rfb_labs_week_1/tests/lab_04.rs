mod support;

use rfb_labs_week_1::labs::lab04_utxos::{
    list_unspent, outpoint, select_spendable_utxo, sum_spendable_utxos,
};
use rfb_labs_week_1::model::{OutPoint, Utxo};
use support::MockRpc;

fn sample_utxos() -> Vec<Utxo> {
    vec![
        Utxo {
            txid: "tx-young".to_owned(),
            vout: 0,
            address: Some("bcrt1qminer".to_owned()),
            script_pub_key: "0014aa".to_owned(),
            amount: 25.0,
            confirmations: 2,
            spendable: false,
        },
        Utxo {
            txid: "tx-mature".to_owned(),
            vout: 1,
            address: Some("bcrt1qminer".to_owned()),
            script_pub_key: "0014bb".to_owned(),
            amount: 50.0,
            confirmations: 101,
            spendable: true,
        },
    ]
}

#[test]
fn decodes_listunspent_response() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        Some("miner"),
        "listunspent",
        &[],
        r#"[{"txid":"tx-mature","vout":1,"address":"bcrt1qminer","scriptPubKey":"0014bb","amount":50.0,"confirmations":101,"spendable":true}]"#,
    );

    assert_eq!(
        list_unspent(&rpc, "miner").unwrap(),
        vec![sample_utxos()[1].clone()]
    );
    rpc.assert_done();
}

#[test]
fn selects_most_confirmed_spendable_utxo() {
    let mut utxos = sample_utxos();
    utxos.push(Utxo {
        txid: "tx-other".to_owned(),
        vout: 2,
        address: None,
        script_pub_key: "0014cc".to_owned(),
        amount: 1.0,
        confirmations: 6,
        spendable: true,
    });

    assert_eq!(
        select_spendable_utxo(&utxos),
        Some(sample_utxos()[1].clone())
    );
}

#[test]
fn constructs_unique_outpoint() {
    assert_eq!(
        outpoint(&sample_utxos()[1]),
        OutPoint {
            txid: "tx-mature".to_owned(),
            vout: 1,
        }
    );
}

#[test]
fn sums_only_spendable_outputs() {
    let mut utxos = sample_utxos();
    utxos.push(Utxo {
        txid: "tx-other".to_owned(),
        vout: 2,
        address: None,
        script_pub_key: "0014cc".to_owned(),
        amount: 1.25,
        confirmations: 6,
        spendable: true,
    });

    assert!((sum_spendable_utxos(&utxos) - 51.25).abs() < 0.000_000_01);
}
