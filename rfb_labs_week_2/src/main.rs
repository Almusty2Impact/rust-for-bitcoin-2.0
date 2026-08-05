//! Small executable for Part 8 of the assignment.

use rfb_labs_week_2::transaction::{InputKind, OutPoint, OutputType, Transaction, TxOutput};

fn main() {
    let mut transaction = Transaction::new(2, 0);

    // Input 1: 70,000 sats
    transaction.add_input(InputKind::Regular {
        previous_output: OutPoint {
            txid: "aaaa1111aaaa1111aaaa1111aaaa1111aaaa1111aaaa1111aaaa1111aaaa1111".into(),
            vout: 0,
        },
        value: 70_000,
        sequence: u32::MAX,
    });

    // Input 2: 50,000 sats
    transaction.add_input(InputKind::Regular {
        previous_output: OutPoint {
            txid: "bbbb2222bbbb2222bbbb2222bbbb2222bbbb2222bbbb2222bbbb2222bbbb2222".into(),
            vout: 1,
        },
        value: 50_000,
        sequence: u32::MAX,
    });

    // Payment output: 90,000 sats to receiver
    transaction.add_output(TxOutput {
        value: 90_000,
        recipient: "bc1qreceiver".into(),
        output_type: OutputType::P2wpkh,
    });

    // Change output: 28,000 sats back to sender (120,000 - 90,000 - 2,000 fee)
    transaction.add_output(TxOutput {
        value: 28_000,
        recipient: "bc1qsender".into(),
        output_type: OutputType::P2wpkh,
    });

    println!("{transaction}");

    // Validate
    match transaction.validate() {
        Ok(()) => println!("Validation: OK"),
        Err(e) => println!("Validation: FAILED ({e})"),
    }
}
