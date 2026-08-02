# Lab 01 — Regtest network inspection

## Commands used

bitcoin-cli -regtest getblockchaininfo
bitcoin-cli -regtest getblockcount
bitcoin-cli -regtest getbestblockhash

## Terminal output

Record chain:
regtest
Block height:
1
Best-block hash:
0dccadbb0d4eb0dc059dfe54819da21e53ce2848b65218e1c967ca18c95c1051
 

## Evidence references

![alt text](evidence/image.png)

## Explanation

**Polar** is a GUI management tool for Bitcoin regtest networks. It orchestrates Docker containers so that you can create, start, stop, and destroy local Bitcoin nodes with a few clicks instead of manual configuration. 

**Docker** provides lightweight, isolated Linux containers. Each Polar node runs inside its own Docker container with its own data directory, port mapping, and configuration. 

**Bitcoin Core** is the reference implementation of the Bitcoin protocol. It validates blocks and transactions, maintains the UTXO set, and exposes an RPC interface that tools like `bitcoin-cli` and our Rust code drive. 

**Regtest** (regression test) is a private Bitcoin network with a trivially low mining difficulty. Blocks are produced instantly on demand via `generatetoaddress`, and every address uses the `bcrt1...` bech32 prefix to make it obvious that the coins have no real-world value. Together these four layers let us experiment with Bitcoin mechanics—maturity, fees, reorgs—without risking real funds or waiting for mainnet confirmations.