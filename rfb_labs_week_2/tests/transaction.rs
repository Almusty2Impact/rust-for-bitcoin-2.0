use rfb_labs_week_2::{
    find_outputs_for_recipient, highest_value_output, BitcoinValue, InputKind, OutPoint,
    OutputType, Transaction, TransactionError, TxOutput,
};

fn regular_input(value: u64) -> InputKind {
    InputKind::Regular {
        previous_output: OutPoint {
            txid: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
            vout: 0,
        },
        value,
        sequence: u32::MAX,
    }
}

fn output(value: u64, recipient: &str) -> TxOutput {
    TxOutput {
        value,
        recipient: recipient.into(),
        output_type: OutputType::P2wpkh,
    }
}

fn coinbase_input(reward: u64) -> InputKind {
    InputKind::Coinbase {
        block_height: 100,
        reward,
    }
}

#[test]
fn valid_regular_transaction_passes_validation() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(120_000));
    transaction.add_output(output(90_000, "bc1qreceiver"));
    transaction.add_output(output(28_000, "bc1qsender"));

    assert_eq!(transaction.validate(), Ok(()));
    assert_eq!(transaction.total_input_value(), 120_000);
    assert_eq!(transaction.total_output_value(), 118_000);
    assert_eq!(transaction.fee(), Ok(2_000));
}

#[test]
fn outputs_cannot_exceed_inputs() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(50_000));
    transaction.add_output(output(60_000, "bc1qreceiver"));

    assert_eq!(
        transaction.validate(),
        Err(TransactionError::OutputsExceedInputs {
            total_inputs: 50_000,
            total_outputs: 60_000,
        })
    );
}

#[test]
fn no_inputs_fails_validation() {
    let mut transaction = Transaction::new(1, 0);
    transaction.add_output(output(10_000, "bc1qreceiver"));

    assert_eq!(transaction.validate(), Err(TransactionError::NoInputs));
}

#[test]
fn no_outputs_fails_validation() {
    let mut transaction = Transaction::new(1, 0);
    transaction.add_input(regular_input(50_000));

    assert_eq!(transaction.validate(), Err(TransactionError::NoOutputs));
}

#[test]
fn zero_value_output_fails_validation() {
    let mut transaction = Transaction::new(1, 0);
    transaction.add_input(regular_input(50_000));
    transaction.add_output(TxOutput {
        value: 0,
        recipient: "bc1qreceiver".into(),
        output_type: OutputType::P2wpkh,
    });

    assert_eq!(
        transaction.validate(),
        Err(TransactionError::ZeroValueOutput)
    );
}

#[test]
fn op_return_zero_value_passes_validation() {
    let mut transaction = Transaction::new(1, 0);
    transaction.add_input(regular_input(50_000));
    transaction.add_output(TxOutput {
        value: 0,
        recipient: "data payload".into(),
        output_type: OutputType::OpReturn,
    });
    transaction.add_output(output(48_000, "bc1qsender"));

    assert_eq!(transaction.validate(), Ok(()));
}

#[test]
fn mixed_coinbase_and_regular_fails() {
    let mut transaction = Transaction::new(1, 0);
    transaction.add_input(coinbase_input(50_000));
    transaction.add_input(regular_input(10_000));
    transaction.add_output(output(58_000, "bc1qreceiver"));

    assert_eq!(
        transaction.validate(),
        Err(TransactionError::CoinbaseMixedWithRegularInputs)
    );
}

#[test]
fn multiple_coinbase_inputs_fails() {
    let mut transaction = Transaction::new(1, 0);
    transaction.add_input(coinbase_input(25_000));
    transaction.add_input(coinbase_input(25_000));
    transaction.add_output(output(48_000, "bc1qreceiver"));

    assert_eq!(
        transaction.validate(),
        Err(TransactionError::MultipleCoinbaseInputs)
    );
}

#[test]
fn empty_txid_fails_validation() {
    let mut transaction = Transaction::new(1, 0);
    transaction.add_input(InputKind::Regular {
        previous_output: OutPoint {
            txid: String::new(),
            vout: 0,
        },
        value: 50_000,
        sequence: u32::MAX,
    });
    transaction.add_output(output(48_000, "bc1qreceiver"));

    assert_eq!(transaction.validate(), Err(TransactionError::InvalidTxid));
}

#[test]
fn valid_coinbase_transaction_passes() {
    let mut transaction = Transaction::new(1, 0);
    transaction.add_input(coinbase_input(50_000_000_000));
    transaction.add_output(output(50_000_000_000, "bc1qminer"));

    assert_eq!(transaction.validate(), Ok(()));
    assert_eq!(transaction.fee(), Ok(0));
}

#[test]
fn input_and_output_totals() {
    let mut transaction = Transaction::new(1, 0);
    transaction.add_input(regular_input(100_000));
    transaction.add_input(regular_input(50_000));
    transaction.add_output(output(80_000, "bc1qa"));
    transaction.add_output(output(40_000, "bc1qb"));

    assert_eq!(transaction.total_input_value(), 150_000);
    assert_eq!(transaction.total_output_value(), 120_000);
    assert_eq!(transaction.fee(), Ok(30_000));
}

#[test]
fn highest_value_output_works() {
    let mut transaction = Transaction::new(1, 0);
    transaction.add_input(regular_input(100_000));
    transaction.add_output(output(30_000, "bc1qa"));
    transaction.add_output(output(80_000, "bc1qb"));
    transaction.add_output(output(10_000, "bc1qc"));

    let highest = highest_value_output(&transaction).unwrap();
    assert_eq!(highest.value, 80_000);
    assert_eq!(highest.recipient, "bc1qb");
}

#[test]
fn find_outputs_for_recipient_filters_correctly() {
    let mut transaction = Transaction::new(1, 0);
    transaction.add_input(regular_input(200_000));
    transaction.add_output(output(90_000, "bc1qalice"));
    transaction.add_output(output(50_000, "bc1qbob"));
    transaction.add_output(output(30_000, "bc1qalice"));

    let alice_outputs = find_outputs_for_recipient(&transaction, "bc1qalice");
    assert_eq!(alice_outputs.len(), 2);
    assert_eq!(alice_outputs[0].value, 90_000);
    assert_eq!(alice_outputs[1].value, 30_000);

    let bob_outputs = find_outputs_for_recipient(&transaction, "bc1qbob");
    assert_eq!(bob_outputs.len(), 1);

    let carol_outputs = find_outputs_for_recipient(&transaction, "bc1qcarol");
    assert_eq!(carol_outputs.len(), 0);
}

#[test]
fn bitcoin_value_trait_for_output() {
    let out = output(150_000_000, "bc1qtest");
    assert_eq!(out.value(), 150_000_000);
    assert!((out.value_in_btc() - 1.5).abs() < f64::EPSILON);
}

#[test]
fn bitcoin_value_trait_for_coinbase_input() {
    let input = coinbase_input(6_250_000_000);
    assert_eq!(input.value(), 6_250_000_000);
}

#[test]
fn bitcoin_value_trait_for_regular_input() {
    let input = regular_input(85_000);
    assert_eq!(input.value(), 85_000);
}

#[test]
fn display_outpoint() {
    let op = OutPoint {
        txid: "abcd".into(),
        vout: 3,
    };
    assert_eq!(format!("{op}"), "abcd:3");
}

#[test]
fn display_transaction_summary() {
    let mut tx = Transaction::new(2, 0);
    tx.add_input(regular_input(100_000));
    tx.add_output(output(90_000, "bc1qreceiver"));

    let s = format!("{tx}");
    assert!(s.contains("version=2"));
    assert!(s.contains("locktime=0"));
    assert!(s.contains("1 inputs"));
    assert!(s.contains("1 outputs"));
    assert!(s.contains("100000 sats"));
    assert!(s.contains("90000 sats"));
    assert!(s.contains("10000 sats")); // fee
}

#[test]
fn display_transaction_invalid_fee_no_panic() {
    let mut tx = Transaction::new(1, 0);
    tx.add_input(regular_input(10_000));
    tx.add_output(output(50_000, "bc1qreceiver"));

    let s = format!("{tx}");
    assert!(s.contains("INVALID"));
}
