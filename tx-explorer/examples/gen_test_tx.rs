use bitcoin::blockdata::witness::Witness;
use bitcoin::consensus::serialize;
use bitcoin::hashes::Hash;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::transaction::{Transaction, TxIn, TxOut, Version};
use bitcoin::CompressedPublicKey;
use bitcoin::{Amount, Network, OutPoint, PublicKey, ScriptBuf, Sequence, Txid};

fn main() {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&[1u8; 32]).unwrap();
    let pk = PublicKey::new(sk.public_key(&secp));
    let compressed = CompressedPublicKey::try_from(pk).unwrap();
    let spk = bitcoin::Address::p2wpkh(&compressed, Network::Regtest).script_pubkey();

    let tx = Transaction {
        version: Version::TWO,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([2u8; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::from_slice(&[
                vec![0x30, 0x44, 0x02, 0x20],
                compressed.to_bytes().to_vec(),
            ]),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(49000),
            script_pubkey: spk,
        }],
        lock_time: LockTime::ZERO,
    };
    println!("SEGWIT={}", hex::encode(serialize(&tx)));

    let pk2 = PublicKey::new(SecretKey::from_slice(&[3u8; 32]).unwrap().public_key(&secp));
    let p2pkh_spk = bitcoin::Address::p2pkh(pk2, Network::Regtest).script_pubkey();
    let tx2 = Transaction {
        version: Version::ONE,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([5u8; 32]),
                vout: 1,
            },
            script_sig: ScriptBuf::from_hex("483045022100aabbccdd").unwrap(),
            sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
            witness: Witness::default(),
        }],
        output: vec![
            TxOut {
                value: Amount::from_sat(30000),
                script_pubkey: p2pkh_spk.clone(),
            },
            TxOut {
                value: Amount::from_sat(19000),
                script_pubkey: p2pkh_spk,
            },
        ],
        lock_time: LockTime::ZERO,
    };
    println!("LEGACY={}", hex::encode(serialize(&tx2)));
}
