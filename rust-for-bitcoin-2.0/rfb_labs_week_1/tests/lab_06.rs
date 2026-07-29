mod support;

use rfb_labs_week_1::labs::lab06_decode::{
    calculate_fee, decode_verbose_transaction, identify_payment_and_change, input_outpoints,
};
use rfb_labs_week_1::model::{
    DecodedInput, DecodedOutput, DecodedTransaction, OutPoint, PaymentAndChange,
};
use support::MockRpc;

fn decoded_transaction() -> DecodedTransaction {
    DecodedTransaction {
        txid: "payment-txid".to_owned(),
        inputs: vec![DecodedInput {
            previous_output: OutPoint {
                txid: "funding-txid".to_owned(),
                vout: 0,
            },
            previous_value: 1.5,
        }],
        outputs: vec![
            DecodedOutput {
                vout: 0,
                value: 1.0,
                address: Some("bcrt1qreceiver".to_owned()),
                script_pub_key_hex: "0014aa".to_owned(),
            },
            DecodedOutput {
                vout: 1,
                value: 0.49999,
                address: Some("bcrt1qchange".to_owned()),
                script_pub_key_hex: "0014bb".to_owned(),
            },
        ],
        vsize: 141,
    }
}

#[test]
fn decodes_inputs_outputs_and_virtual_size() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        None,
        "getrawtransaction",
        &["payment-txid", "2"],
        r#"{"txid":"payment-txid","vsize":141,"vin":[{"txid":"funding-txid","vout":0,"prevout":{"value":1.5}}],"vout":[{"value":1.0,"n":0,"scriptPubKey":{"hex":"0014aa","address":"bcrt1qreceiver"}},{"value":0.49999,"n":1,"scriptPubKey":{"hex":"0014bb","address":"bcrt1qchange"}}]}"#,
    );

    assert_eq!(
        decode_verbose_transaction(&rpc, "payment-txid").unwrap(),
        decoded_transaction()
    );
    rpc.assert_done();
}

#[test]
fn returns_consumed_outpoints() {
    assert_eq!(
        input_outpoints(&decoded_transaction()),
        vec![OutPoint {
            txid: "funding-txid".to_owned(),
            vout: 0,
        }]
    );
}

#[test]
fn distinguishes_receiver_output_from_change() {
    let transaction = decoded_transaction();
    assert_eq!(
        identify_payment_and_change(&transaction, "bcrt1qreceiver").unwrap(),
        PaymentAndChange {
            payment: transaction.outputs[0].clone(),
            change: Some(transaction.outputs[1].clone()),
        }
    );
}

#[test]
fn calculates_fee_from_input_and_output_values() {
    let fee = calculate_fee(&decoded_transaction()).unwrap();
    assert!((fee - 0.00001).abs() < 0.000_000_001);
}
