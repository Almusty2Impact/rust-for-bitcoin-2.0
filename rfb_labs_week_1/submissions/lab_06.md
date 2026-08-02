# Lab 06 — Transaction decoding

## Commands used

bitcoin-cli -regtest getrawtransaction 2f84a7d33c1c6b4c1bb00dca018c953400f25b31a0d0f22b8779817e1fdb4b0a 2

## Terminal output

{
  "txid": "2f84a7d33c1c6b4c1bb00dca018c953400f25b31a0d0f22b8779817e1fdb4b0a",
  "hash": "7105389e68d2be7c8f033c006afa6159a5fd065b4fb92809502d1db8a217c57b",
  "version": 2,
  "size": 167,
  "vsize": 140,
  "weight": 560,
  "locktime": 1,
  "vin": [
    {
      "coinbase": "5200",
      "txinwitness": [
        "0000000000000000000000000000000000000000000000000000000000000000"
      ],
      "sequence": 4294967294
    }
  ],
  "vout": [
    {
      "value": 50.00000000,
      "n": 0,
      "scriptPubKey": {
        "asm": "0 3b81090c90364aef866ecfa6843cb4a79fc045e7",
        "desc": "addr(bcrt1q8wqsjrysxe9wlpnwe7ngg095570uq308nktqcl)#wxmqphqq",
        "hex": "00143b81090c90364aef866ecfa6843cb4a79fc045e7",
        "address": "bcrt1q8wqsjrysxe9wlpnwe7ngg095570uq308nktqcl",
        "type": "witness_v0_keyhash"
      }
    },
    {
      "value": 0.00000000,
      "n": 1,
      "scriptPubKey": {
        "asm": "OP_RETURN aa21a9ede2f61c3f71d1defd3fa999dfa36953755c690689799962b48bebd836974e8cf9",
        "desc": "raw(6a24aa21a9ede2f61c3f71d1defd3fa999dfa36953755c690689799962b48bebd836974e8cf9)#cav96mf3",
        "hex": "6a24aa21a9ede2f61c3f71d1defd3fa999dfa36953755c690689799962b48bebd836974e8cf9",
        "type": "nulldata"
      }
    }
  ],
  "hex": "020000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff025200feffffff0200f2052a010000001600143b81090c90364aef866ecfa6843cb4a79fc045e70000000000000000266a24aa21a9ede2f61c3f71d1defd3fa999dfa36953755c690689799962b48bebd836974e8cf90120000000000000000000000000000000000000000000000000000000000000000001000000",
  "blockhash": "61ca3102c4836627a9ecef8eb98222d6465b5465599cd128ad5a3087f67fed30",
  "confirmations": 103,
  "time": 1785596173,
  "blocktime": 1785596173
}

## Evidence references

![alt text](evidence/image-8.png)

## Explanation

The fee is not a dedicated transaction output. In Bitcoin, the miner fee is the unassigned difference between the total value consumed by the inputs and the total value allocated to the outputs. When a wallet constructs a transaction, it selects UTXOs worth more than the intended payment, sends the exact payment amount to the recipient, returns the surplus to itself as a change output, and whatever tiny remainder is left after accounting for both outputs becomes the fee. Miners collect this difference by including the transaction in a block—their reward is the block subsidy plus the sum of all transaction fees in that block.

This design means there is no explicit "fee output" in the transaction data. The fee is implicit: it is simply the gap between inputs and outputs. This is why the transaction is valid only if sum(inputs) >= sum(outputs), and a transaction where outputs exceed inputs is invalid. The fee cannot be separately assigned or earmarked; it is the residual that incentivizes miners to include the transaction in a block. The virtual size (vsize) of 141 bytes and the fee of 0.00001 BTC gives a fee rate of approximately 70.9 sat/vB.