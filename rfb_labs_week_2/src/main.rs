//! Small executable for Part 8 of the assignment.

use rfb_labs_week_2::transaction::Transaction;

fn main() {
    let transaction = Transaction::new(2, 0);

    // TODO(Part 8): add the two supplied UTXOs as inputs, then add the
    // payment and correctly calculated change outputs.
    println!("{transaction}");
}
