# Week 1 Practical Assessment

**Platform:** Polar, Docker, Bitcoin Core, and regtest
**Labs:** 10
**Final score:** 100 points

For every lab:

1. Implement the four functions in the matching `src/labs/labXX_*.rs` file.
2. Pass the matching public test suite.
3. Run the completed work against Bitcoin Core in Polar.
4. Complete the matching `submissions/lab_XX.md` evidence file.

Use fake regtest coins only. Never submit private keys, seed phrases, authentication
cookies, or information from a mainnet wallet.

## Lab 01 — Build and verify a regtest network

Create a Polar network named **Week 1 Bitcoin Fundamentals**, add a Bitcoin Core
node, and start it. Use your Rust implementation to prove:

- the node is running;
- the selected chain is `regtest`;
- the current block height;
- the current best-block hash.

Explain the distinct roles of Polar, Docker, Bitcoin Core, and regtest.

## Lab 02 — Create wallets and addresses

Create wallets named `miner` and `receiver`. Generate a `mining` address from the
miner wallet and a `classmate` address from the receiver wallet. Prove:

- both wallets are loaded;
- both addresses use the `bcrt1...` regtest prefix;
- each address belongs to the expected wallet.

Explain why wallet-scoped calls need `-rpcwallet` and what a wrong wallet context
means.

## Lab 03 — Demonstrate coinbase maturity

Mine one block to the miner address. Inspect the trusted and immature balances, then
attempt to send 1 BTC to the receiver before mining more blocks. Preserve the error.

Mine 100 more blocks. Prove that:

- the chain reached height 101;
- the first coinbase reward is now spendable;
- later rewards remain immature.

Explain the `COINBASE_MATURITY = 100` rule and why the lab conventionally mines 101
blocks on a fresh chain.

## Lab 04 — Inspect a UTXO and its outpoint

List the miner wallet's unspent outputs. Choose one spendable output and record:

- `txid`;
- `vout`;
- amount;
- confirmations;
- address and locking script;
- spendable state.

Construct its outpoint and independently sum all spendable UTXOs. Reconcile that sum
with Bitcoin Core's wallet balance. Explain why a wallet balance is not an account
entry.

## Lab 05 — Broadcast and observe an unconfirmed payment

Send exactly 1 BTC from the miner wallet to the receiver, but do not mine. Preserve
the returned TXID and prove:

- the TXID appears in the node's local mempool;
- the sender reports zero confirmations;
- the receiver can see an untrusted-pending balance;
- broadcast is not confirmation.

Describe the transaction states: built and signed, broadcast, mempool, and confirmed.

## Lab 06 — Decode and audit value conservation

Decode the unconfirmed transaction with verbosity sufficient to expose each input's
previous output. Identify:

- every consumed `txid:vout`;
- every new output;
- the receiver's 1 BTC payment output;
- the sender's change output;
- virtual size;
- miner fee.

Show with actual values:

```text
sum(inputs) = sum(payment outputs) + sum(change outputs) + fee
```

Explain why the fee is the unassigned difference rather than a dedicated transaction
output.

## Lab 07 — Confirm and locate the transaction

Mine exactly one block. Prove:

- the TXID left the mempool;
- the receiver's balance became trusted;
- the transaction has one confirmation;
- Bitcoin Core reports a containing block hash;
- that block's transaction list contains the TXID.

Explain whether mining changed the serialized transaction or its place in the agreed
history.

## Lab 08 — Inspect block commitments and confirmation depth

Inspect the confirming block's verbose header and record:

- block hash and height;
- previous-block hash;
- Merkle root;
- nonce;
- bits, target, or difficulty representation;
- confirmations;
- accumulated chainwork.

Mine five additional blocks and prove the payment now has six confirmations. Explain
hash links, Merkle commitment, proof-of-work search, and why confirmations increase
reorganization cost without making an invalid transaction valid.

## Lab 09 — Force multi-UTXO coin selection

Create an `alice` wallet. Send Alice three separate 0.4 BTC payments and confirm
them. Prove Alice owns three distinct UTXOs.

Have Alice send 1 BTC to a new receiver address. Decode the spend and prove:

- more than one input was required;
- selected inputs were consumed completely;
- the receiver received 1 BTC;
- surplus returned as change;
- the difference is the fee.

Explain why combining UTXOs can reveal common ownership and therefore create a
privacy trade-off.

## Lab 10 — Observe competing branches and a reorganization

Add and synchronize a second Bitcoin Core node. Record a common height and
best-block hash. Disconnect the nodes, then:

- mine two blocks privately on Node A;
- mine four blocks privately on Node B;
- record both private tips and their chainwork;
- reconnect the nodes;
- wait for synchronization;
- prove both nodes converge on the same tip.

Explain why one branch became stale, what a reorganization is, and why nodes choose
the valid branch with the greatest accumulated work rather than a miner identity,
arrival time, or social claim.

## Marking

Each lab is worth ten points:

| Category | Points | Assessment |
|---|---:|---|
| Correct execution | 4 | One point per passing public Rust test |
| Commands and evidence | 3 | One point per completed required evidence section |
| Explanation | 3 | Instructor review for correctness and clarity |

GitHub Actions reports the automated portion out of 70. The instructor adds the
explanation portion out of 30.
