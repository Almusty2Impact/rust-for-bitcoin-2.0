# Rust for Bitcoin 2.0 — Week 2

Build a simplified Bitcoin transaction model while practising structs, enums,
traits, ownership, borrowing, collections, and `Result`-based error handling.

The crate is intentionally incomplete. Search for `TODO` and implement each part;
do not change the public type names or function signatures.

## Recommended workflow

1. Read [ASSIGNMENT.md](ASSIGNMENT.md).
2. Complete Parts 3–5 in `transaction.rs` and `error.rs`.
3. Remove `#[ignore]` from the relevant test and run it.
4. Complete the traits and borrowing functions in Parts 6–7.
5. Build the payment example in `main.rs`.
6. Complete UTXO selection and its tests.
7. Add the remaining required tests yourself.

```bash
cargo test
cargo test -- --ignored
cargo run
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

`cargo test` checks the starter project. Ignored tests intentionally exercise
unfinished code; enable them progressively rather than leaving them ignored in the
submission.

## Written answers

### 1. What is a Bitcoin transaction input?

A transaction input references a previous transaction output that the spender wishes to consume. Each regular input contains an outpoint (the txid and vout index of the output being spent), a sequence number (used for RBF and relative locktimes), and a unlocking script (signature + public key) that proves the spender is authorized to spend that output. Coinbase inputs are a special case: they have no previous output to reference, and instead declare a block height and mining reward.

### 2. What is a Bitcoin transaction output?

A transaction output locks a specific amount of bitcoin to a spending condition. Each output contains a value (in satoshis), a locking script (scriptPubKey) that defines who can spend it, and implicitly an output type (P2PKH, P2WPKH, P2TR, or OP_RETURN). Outputs are not "owned" by anyone until someone satisfies the locking script. In this simplified model, we represent the locking condition as a recipient address string and an `OutputType` enum.

### 3. What is a UTXO?

A UTXO (Unspent Transaction Output) is a transaction output that has not yet been consumed by any subsequent transaction input. UTXOs are the fundamental unit of spendable bitcoin. The global UTXO set, maintained by all full nodes, is the closest thing Bitcoin has to a "balance ledger." A wallet's balance is the sum of the values of all UTXOs whose locking scripts the wallet can satisfy. When a transaction is confirmed, the inputs it consumes are removed from the UTXO set, and its new outputs are added.

### 4. What does an outpoint identify?

An outpoint uniquely identifies a specific output within a specific transaction. It is the pair (txid, vout) — the 32-byte transaction hash plus the zero-based output index. Together, these two values form a globally unique coordinate into the UTXO set. When a transaction spends an output, it references the output's outpoint in its input, proving which coin is being consumed.

### 5. How is a transaction fee calculated?

The transaction fee is the difference between the total value of all inputs and the total value of all outputs: `fee = total_inputs - total_outputs`. The fee is not a separate output — it is the unassigned residual. Miners collect this difference as part of their block reward. A transaction is invalid if outputs exceed inputs (negative fee), and has zero fee if inputs exactly equal outputs.

### 6. Why use integers rather than floating-point numbers for bitcoin amounts?

Floating-point numbers (f64) cannot precisely represent all decimal values. For example, 0.1 + 0.2 != 0.3 in IEEE 754 arithmetic. Bitcoin operates in satoshis (1 BTC = 100,000,000 sats), which are integer units. Using `u64` for satoshi amounts ensures exact arithmetic with no rounding errors. Financial calculations must be deterministic — a fee miscalculation of even 1 satoshi could cause a transaction to be rejected by the network. Integers give us that precision.

### 7. Why does `total_input_value()` borrow `self`?

`total_input_value()` takes `&self` because it only needs to *read* the transaction's inputs to compute a sum — it does not modify the transaction. Borrowing allows the caller to retain ownership and continue using the transaction after the call. If it took `self` by value, the caller would lose access to their transaction, which would be unnecessarily restrictive for a simple read-only computation.

### 8. Why does `add_input()` take `&mut self`?

`add_input()` needs to *modify* the transaction by pushing a new element onto its `inputs` vector. In Rust, mutation requires a mutable reference (`&mut self`). This signals to both the compiler and the reader that this method changes the transaction's state. It also means no other references to the transaction can exist simultaneously, preventing data races at compile time.

### 9. What happens when an input is moved into a transaction?

When `add_input(input)` is called, the `input` value is moved into the transaction's vector via `self.inputs.push(input)`. After the move, the caller no longer owns the input and cannot use it again. This is Rust's ownership system in action: the transaction now exclusively owns that input. There is no shallow copy or shared reference — the data has been transferred. To use the same input data in two transactions, the caller would need to clone it first.

### 10. Why is `Result` preferable to `panic!` for validation failures?

A `panic!` crashes the program immediately and cannot be caught gracefully. Validation failures like "outputs exceed inputs" are *expected* conditions that calling code should handle — not unrecoverable bugs. `Result<T, E>` forces the caller to explicitly acknowledge the possibility of failure (using `?`, `match`, or `.unwrap()`), making error handling visible and deliberate. This is especially important in a library: library code should never panic on expected invalid inputs, because the library user deserves the chance to respond appropriately.

### 11. How do enums help model regular and coinbase inputs?

Bitcoin transactions have exactly two kinds of inputs: regular inputs (which reference a previous output via an outpoint) and coinbase inputs (which claim a block reward and have no previous output). These have fundamentally different fields — a regular input has a `previous_output` and `sequence`, while a coinbase input has a `block_height` and `reward`. A Rust enum with two variants lets us represent this "one of two shapes" relationship directly, with each variant carrying only its relevant data. When we `match` on `InputKind`, the compiler guarantees we have handled both cases, so we cannot accidentally treat a coinbase input as if it had a `previous_output` field. This is safer than using a single struct with optional fields.

### 12. How does the `BitcoinValue` trait reduce duplication?

Both `TxOutput` and `InputKind` represent things that have a satoshi value, but they store it in different fields (`value` for outputs, `value`/`reward` for the two input variants). Without a trait, any code that needs to read the value would need separate logic for each type. The `BitcoinValue` trait defines a single `value()` method that each type implements, and provides a shared `value_in_btc()` conversion. This means generic code can call `.value()` on anything that implements `BitcoinValue` — whether it's an output, a regular input, or a coinbase input — without knowing or caring about the concrete type.

### Ownership compiler error (Part 7)

If we attempted to implement `highest_value_output` by returning an owned `TxOutput` instead of a reference, and then tried to also return that same output from the transaction later, the compiler would reject it because the output has already been moved:

```text
error[E0382]: use of moved value: `transaction.outputs[0]`
  --> src/transaction.rs:112:22
   |
11 |     let best = transaction.outputs.into_iter().max_by_key(|o| o.value).unwrap();
   |                 -------------------- value moved here
12 |     let _second = &transaction.outputs[0];
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^ value borrowed here after move
```

This happens because `into_iter()` consumes the vector, moving each element out. After the move, the original vector is partially moved and no longer accessible. The fix is to use `iter()` instead, which borrows each element, returning `&TxOutput` references and leaving the original data intact.

## Design notes

**UTXO selection algorithm:** The basic implementation selects UTXOs in slice order until the cumulative value meets or exceeds the target. This is simple and deterministic but not optimal. A greedy "smallest-first" approach (ascending order by value) would minimize the number of UTXOs consumed, reducing transaction size and therefore fees. An ascending-by-value sort would be a straightforward improvement. For production wallets, more sophisticated algorithms exist (e.g., branch-and-bound for exact matches, or Knapsack-based approximation) that balance privacy, fee minimization, and UTXO pool fragmentation.

**Fee as `checked_sub`:** The `fee()` method uses `u64::checked_sub` to avoid integer underflow. Rather than panicking on `outputs > inputs`, we return `OutputsExceedInputs` as a proper error, allowing callers to handle the condition gracefully.

**Display for invalid transactions:** The `Display` impl for `Transaction` calls `self.fee()` and handles the error case by printing "INVALID" instead of panicking. This means formatting a bad transaction for debugging never crashes the program.

**Optional state machine (Part 10):** Not implemented in this submission. A state-machine extension would use an enum like `enum TxState { Created, Validated, Signed, Broadcast, Confirmed, Rejected }` and wrap the `Transaction` struct so that methods like `sign()` are only available in the `Validated` state, and `broadcast()` only in the `Signed` state. This would be enforced at compile time using Rust's type system (e.g., a generic `Transaction<S>` parameterized by state).

## Example output

```
Transaction(version=2, locktime=0)
  2 inputs
    Regular(aaaa1111aaaa1111aaaa1111aaaa1111aaaa1111aaaa1111aaaa1111aaaa1111:0) for 70000 sats (seq=4294967295)
    Regular(bbbb2222bbbb2222bbbb2222bbbb2222bbbb2222bbbb2222bbbb2222bbbb2222:1) for 50000 sats (seq=4294967295)
  2 outputs
    90000 sats -> bc1qreceiver (P2wpkh)
    28000 sats -> bc1qsender (P2wpkh)
  total input:  120000 sats
  total output: 118000 sats
  fee:          2000 sats

Validation: OK
```