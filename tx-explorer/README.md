<div align="center">

# ⚡ txexp

### Bitcoin Transaction Explorer

*Decode · Analyse · Inspect — one transaction at a time.*

[![Rust](https://img.shields.io/badge/rust-1.73%2B-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![Bitcoin](https://img.shields.io/badge/bitcoin-mainnet%20%7C%20testnet4%20%7C%20signet%20%7C%20regtest-f7931a?style=for-the-badge&logo=bitcoin)](https://bitcoin.org)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-10%20passing-brightgreen?style=for-the-badge)](#-development)

**A microscope for a single Bitcoin transaction — not a map of the chain.**

[Features](#-features) · [Install](#-install) · [Usage](#-usage) · [Examples](#-example-output) · [Architecture](#-architecture) · [Troubleshooting](#-troubleshooting)

</div>

---

## 🎯 What is this?

`txexp` takes **one** Bitcoin transaction — as raw hex or a txid — and breaks it down field by field. It decodes every byte, identifies script types, disassembles opcodes, derives addresses, and computes size, weight, vsize, fee, and fee rate.

> **Not a block explorer.** It doesn't browse the chain or track addresses. It goes *deep* into the structure of a single transaction — built for learning, debugging, and understanding how Bitcoin actually works under the hood.

---

## ✨ Features

|   | Feature |
|:-:|:--------|
| 🔤 | Accept a **txid** or **raw transaction hex** |
| 🧩 | Decode **version**, **inputs** (outpoint, scriptSig, witness, sequence), **outputs** (value, scriptPubKey), **locktime** |
| 🏷️ | Identify script types — **P2PKH · P2SH · P2WPKH · P2WSH · P2TR · OP_RETURN · P2PK** |
| 📮 | Derive **addresses** from output scripts |
| 🔍 | **Disassemble scripts** into human-readable opcodes |
| ⚖️ | Calculate **size · weight · vsize · fee · fee rate** (sat/vB) |
| 🪶 | Detect **SegWit** variant (Native / Mixed / None) and **RBF** signalling |
| ⛓️ | Show **confirmation status** |
| 🪙 | Handle **coinbase transactions** as a special case |
| 🌐 | **Two data backends** — Bitcoin Core RPC *(self-hosted)* or mempool.space *(public)* |
| 🎨 | Coloured terminal tables **or** `--json` output |
| 🕸️ | Network selection — `regtest · signet · testnet4 · bitcoin` |

---

## 📦 Install

```bash
git clone https://github.com/<your-username>/tx-explorer.git
cd tx-explorer
cargo build --release
cp target/release/txexp ~/.cargo/bin/txexp
```

**Requirements**

| Backend | Needs |
|:--------|:------|
| `hex` — offline decode | *Nothing* — works fully offline |
| `api` — mempool.space | Outbound HTTPS |
| `txid` — Bitcoin Core RPC | A node running with `txindex=1` |

---

## 🚀 Quick Start

```bash
# 1 ─ Offline decode
txexp hex 02000000000101a0f2da16d33d74f1e45f53a35bf80e...

# 2 ─ Same tx, JSON
txexp --json hex 02000000000101a0f2da16d33d74f1e45f53a35bf80e...

# 3 ─ Online lookup, no node required
txexp --network bitcoin api <TXID>

# 4 ─ Fetch from your own Bitcoin Core node
txexp --network regtest txid <TXID>
```

---

## 🛠 Usage

### `hex` — decode raw transaction hex

<span title="offline">📴</span> **Fully offline.** No network, no RPC, no third party.

```bash
txexp hex <RAW_HEX>
txexp --json hex <RAW_HEX>
txexp --network bitcoin hex <RAW_HEX>
```

> ℹ️ The fee line reads `— (requires RPC to fetch previous outputs)`. **That's expected.** A raw transaction does not contain input amounts — see [Key Notes](#-key-notes).

`decode` is accepted as an alias.

### `api` — fetch by txid via mempool.space

<span title="online">🌐</span> **Requires outbound HTTPS.** No node, no Docker, no credentials.

```bash
txexp --network bitcoin api <TXID>
txexp --json --network bitcoin api <TXID>
```

Fetches the raw tx, decodes it locally, then fetches fee and confirmation status from the Esplora API. **The fastest path to a full demo.**

### `txid` — fetch by txid via Bitcoin Core RPC

<span title="self-hosted">🖥️</span> **Self-hosted.** Bring your own node.

```bash
cp .env.example .env
txexp --network regtest txid <TXID>
```

Credentials loaded from `.env` via `dotenvy`:

```ini
RPC_URL=http://127.0.0.1:18443
RPC_USER=your_user
RPC_PASS=your_pass
```

> ⚠️ Your node needs `txindex=1` for non-wallet transactions to be fetchable by txid.

`fetch` is accepted as an alias.

### Global options

| Flag | Alias | Values | Default |
|:-----|:-----:|:-------|:-------:|
| `--json` | `-j` | — | `off` |
| `--network` | `-n` | `regtest` `signet` `testnet4` `bitcoin` | `regtest` |
| `--help` | `-h` | — | — |
| `--version` | `-V` | — | — |

> 💡 **Use `--network bitcoin` for mainnet txids.** On the wrong network, base58 P2PKH/P2SH addresses show `—`. Bech32 (SegWit) addresses still render.

---

## 🖼 Example Output

```text
⚡ Transaction Explorer ──────────────────────
  TXID: 905ecdf95a84804b192f4dc221cfed4d77959b81ed66013a7e41a6e61e7ed530
  WTXID: 526a41ff7182577e3eff0af07b399d310e4d33def08eb847322d818f80cd0738
  Version: 2   Locktime: 0   Coinbase: no
  SegWit: Native   RBF: yes
  Confirmations: 260117 confirmed

╭───────────────────┬────────────╮
│ Metric            ┆ Value      │
╞═══════════════════╪════════════╡
│ Size (stripped)   ┆ 120        │
│ Total size        ┆ 389        │
│ Weight (WU)       ┆ 749        │
│ Virtual size (vB) ┆ 188        │
│ Witness discount  ┆ -269 bytes │
╰───────────────────┴────────────╯

  Fee: 7500 sats   Fee rate: 39.89 sat/vB
  Inputs sum: 7500 sats   Outputs sum: 0 sats

📥 Inputs (1):
  #0  5b79f5d6...201bb4:0   scriptSig=∅   witness=4 items   seq=0xfffffffd   RBF=✓

📤 Outputs (1):
  #0  0 BTC   OpReturn   —   OP_RETURN OP_PUSHBYTES_58 54687820536174…
```

> 📌 This tx is a **data-carrier burn** — one input, one `OP_RETURN` output with value 0, the entire input paid as fee. `OP_RETURN` outputs are unspendable by design and always carry value 0.

---

## 📚 Where to Get Txids and Raw Hex

| Source | How |
|:-------|:----|
| 🌐 [mempool.space](https://mempool.space) | Search any address or txid — txid is in the URL |
| 🌐 [blockstream.info](https://blockstream.info) | Add `/hex` to a tx URL → `/tx/<TXID>/hex` |
| 🌐 [blockchain.com](https://www.blockchain.com/explorer) | Search a transaction |
| 🖥️ Bitcoin Core | `bitcoin-cli getrawtransaction <txid>` |

---

## 🏗 Architecture

```text
   CLI ──▶ decode ──▶ analysis ──▶ output
    │        │           │           ▲
    │        ▼           ▼           │
    └──▶ script ──▶ fees ──────▶ (result)
                ▲
                │
        rpc ────┴──── mempool
        (node)        (public)
```

| Module | Responsibility |
|:-------|:---------------|
| `main.rs` | CLI entry point · orchestration |
| `lib.rs` | Public API re-exports |
| `decode.rs` | Consensus-decode raw bytes → `DecodedTransaction` |
| `script.rs` | Script type detection + address derivation |
| `analysis.rs` | SegWit variant · RBF · size/weight/vsize |
| `fees.rs` | Fee calculation from input/output sums |
| `rpc.rs` | Bitcoin Core RPC client |
| `mempool.rs` | mempool.space Esplora client |
| `output.rs` | Terminal tables or JSON |
| `error.rs` | Typed `ExplorerError` enum |

> 🧱 **Error strategy.** Library uses `thiserror` for typed errors; CLI converts them to ergonomic `anyhow` chains with `.context()`.

---

## 🧰 Crates

| Crate | Purpose |
|:------|:--------|
| `bitcoin` 0.32 | Transactions · scripts · addresses · weight/vsize |
| `bitcoincore-rpc` 0.19 | Fetch tx + prevouts from Bitcoin Core |
| `ureq` 2 | Sync HTTPS client for mempool.space |
| `clap` 4 | Subcommands · flags · help |
| `comfy-table` 7 | Table output |
| `owo-colors` 4 | Coloured terminal output |
| `serde` / `serde_json` | JSON serialisation |
| `thiserror` 2 · `anyhow` 1 | Error handling |
| `dotenvy` 0.15 | Load `.env` |
| `tokio` 1 | Async runtime (RPC path) |
| `hex` 0.4 | Hex encoding/decoding |

---

## 🧪 Development

```bash
cargo test                       # 10 integration tests
cargo fmt --check                # Check formatting
cargo clippy -- -D warnings      # Lint
cargo run --example gen_test_tx  # Print sample SEGWIT= / LEGACY= hex
```

**Test coverage:** SegWit decode · legacy vs SegWit detection · P2PKH & P2WPKH classification · fee + fee rate · coinbase fee = 0 · SegWit variant · RBF detection · JSON round-trip.

---

## 🔑 Key Notes

### 💡 A raw transaction does not contain input values

A Bitcoin transaction references previous outputs by outpoint (`txid:vout`), but **never stores their value**. The value only exists on the *previous* transaction's output.

```text
fee = sum(input values) − sum(output values)
```

This is why `txexp` has **two backends**:

| Backend | Where input values come from |
|:--------|:-----------------------------|
| `txid` | Bitcoin Core node via RPC |
| `api` | mempool.space Esplora API |

Both produce the same `FeeAnalysis` struct through the same `fees::calculate_fee` function. Only the source of the inputs differs.

> ❌ `hex` cannot resolve prevouts — it always shows `Fee: — (requires RPC to fetch previous outputs)`.

### 🪙 Coinbase transactions are a special case

Coinbase txs have no real inputs (null outpoint) and create new bitcoins. Fee calculation does not apply — the tool reports `fee = 0` and marks the tx accordingly.

### ⚖️ Weight and virtual size

```text
weight = stripped_size × 3 + total_size     (weight units, WU)
vsize  = ceil(weight / 4)                    (virtual bytes, vB)
```

SegWit gives witness bytes a **4× discount** — witness bytes count as 1 WU, non-witness as 4. That's why SegWit txs have lower vsize and therefore lower fees at the same sat/vB.

### 🦀 rust-bitcoin 0.32 API notes

- Use `tx.compute_txid()` / `tx.compute_wtxid()` — `txid()` is deprecated
- Classifiers: `is_p2pkh()` `is_p2sh()` `is_p2wpkh()` `is_p2wsh()` `is_p2tr()` `is_op_return()` `is_p2pk()`
- `OP_0` is `OP_PUSHBYTES_0`; `OP_1`..`OP_16` are `OP_PUSHNUM_1`..`OP_PUSHNUM_16`
- Opcodes aren't `Ord`-comparable — use `to_u8()`

---

## 🩺 Troubleshooting

<details>
<summary><b>🔴 <code>txexp api</code> hangs or times out</b></summary>

Check outbound HTTPS first:

```bash
curl -s -m 10 https://mempool.space/api/blocks/tip/height
```

If curl fails too:

- **WSL2** — the virtual network stack can drop after host sleep. Fix with `wsl --shutdown` in PowerShell, wait 10s, reopen WSL.
- **Corporate / VPN** — set `HTTPS_PROXY`.
- **Firewall** — allow outbound TCP 443 from WSL.

If curl works but `txexp` doesn't, verify `ureq` is in `Cargo.lock` (not `minreq`) — that's an older build.

</details>

<details>
<summary><b>🔴 <code>txexp txid</code> returns HTTP 401</b></summary>

Wrong RPC credentials. Verify independently:

```bash
curl -s --user <user>:<pass> \
  --data-binary '{"jsonrpc":"1.0","id":"t","method":"getblockcount","params":[]}' \
  -H 'content-type: text/plain;' \
  $RPC_URL/
```

If that returns a number, credentials are right and the issue is env-var precedence. `dotenvy` **does not** override exported variables. Run:

```bash
unset RPC_URL RPC_USER RPC_PASS
```

</details>

<details>
<summary><b>🔴 Transaction not found</b></summary>

Either the txid isn't on your node's chain (regtest vs mainnet mix-up), or the node lacks `txindex=1`. Verify:

```bash
bitcoin-cli getrawtransaction <txid>
```

</details>

<details>
<summary><b>🔴 Addresses show <code>—</code></b></summary>

Wrong network. Pass `--network bitcoin` (or `testnet4` / `signet`).

</details>

---

<div align="center">

### 📄 License

**MIT** — see [LICENSE](LICENSE)

<br>

*Built with 🦀 Rust and an unreasonable amount of curiosity about Bitcoin internals.*

</div>
