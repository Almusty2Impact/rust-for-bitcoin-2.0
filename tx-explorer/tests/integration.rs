//! Integration tests for tx-explorer.

use bitcoin::blockdata::witness::Witness;
use bitcoin::consensus::serialize;
use bitcoin::hashes::Hash;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::transaction::{Transaction, TxIn, TxOut, Version};
use bitcoin::{
    Amount, CompressedPublicKey, Network, OutPoint, PublicKey, ScriptBuf, Sequence, Txid,
};

use tx_explorer::analysis;
use tx_explorer::decode::{build_decoded_tx, decode_hex};
use tx_explorer::fees::calculate_fee;
use tx_explorer::output::ExplorationResult;
use tx_explorer::script::classify_outputs;
use tx_explorer::script::{identify_script_type, ScriptType};

fn make_segwit_tx() -> Transaction {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&[1u8; 32]).unwrap();
    let pk = PublicKey::new(sk.public_key(&secp));
    let compressed = CompressedPublicKey::try_from(pk).unwrap();
    let spk = bitcoin::Address::p2wpkh(&compressed, Network::Regtest).script_pubkey();

    Transaction {
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
    }
}

fn make_legacy_tx() -> Transaction {
    let secp = Secp256k1::new();
    let pk = PublicKey::new(SecretKey::from_slice(&[3u8; 32]).unwrap().public_key(&secp));
    let p2pkh_spk = bitcoin::Address::p2pkh(pk, Network::Regtest).script_pubkey();

    Transaction {
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
    }
}

#[test]
fn decodes_a_segwit_transaction_from_hex() {
    let tx = make_segwit_tx();
    let hex = hex::encode(serialize(&tx));
    let decoded_tx = decode_hex(&hex).unwrap();
    assert_eq!(decoded_tx.compute_txid(), tx.compute_txid());
}

#[test]
fn identifies_segwit_transaction() {
    let tx = make_segwit_tx();
    let decoded = build_decoded_tx(&tx);
    assert!(decoded.has_witness);
    assert!(!decoded.is_coinbase);
    assert_eq!(decoded.version, 2);
}

#[test]
fn identifies_legacy_p2pkh_transaction() {
    let tx = make_legacy_tx();
    let decoded = build_decoded_tx(&tx);
    assert!(!decoded.has_witness);
    assert!(decoded.signals_rbf);
    assert_eq!(decoded.outputs.len(), 2);
}

#[test]
fn classifies_p2wpkh_output() {
    let spk_hex = "001479b000887626b294a914501a4cd226b58b235983";
    let bytes = hex::decode(spk_hex).unwrap();
    let script = ScriptBuf::from_bytes(bytes);
    assert_eq!(identify_script_type(&script), ScriptType::P2wpkh);
}

#[test]
fn classifies_p2pkh_output() {
    let spk_hex = "76a91462e907b2e48e40e5f0aee8eef9f31c2a3b0a5a0888ac";
    let bytes = hex::decode(spk_hex).unwrap();
    let script = ScriptBuf::from_bytes(bytes);
    assert_eq!(identify_script_type(&script), ScriptType::P2pkh);
}

#[test]
fn calculates_fee_correctly() {
    let fee = calculate_fee(50000, 49000, 93, false);
    assert_eq!(fee.fee_sats, 1000);
    assert!((fee.fee_rate_sat_vb - 10.75).abs() < 0.01);
}

#[test]
fn coinbase_fee_is_zero() {
    let fee = calculate_fee(0, 5000000000, 200, true);
    assert_eq!(fee.fee_sats, 0);
    assert!(fee.is_coinbase);
}

#[test]
fn analysis_detects_segwit_variant() {
    let tx = make_segwit_tx();
    let decoded = build_decoded_tx(&tx);
    let a = analysis::analyze(&decoded);
    assert_eq!(a.segwit_variant, "Native");
}

#[test]
fn analysis_detects_rbf() {
    let tx = make_legacy_tx();
    let decoded = build_decoded_tx(&tx);
    let a = analysis::analyze(&decoded);
    assert!(a.signals_rbf);
}

#[test]
fn json_output_is_valid() {
    let tx = make_segwit_tx();
    let decoded = build_decoded_tx(&tx);
    let scripts = classify_outputs(
        &decoded
            .outputs
            .iter()
            .map(|o| o.script_pubkey_hex.clone())
            .collect::<Vec<_>>(),
        Network::Regtest,
    );
    let a = analysis::analyze(&decoded);
    let result = ExplorationResult {
        decoded,
        script_info: scripts,
        fee: None,
        analysis: a,
        confirmations: None,
    };
    let json = result.to_json().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed["decoded"]["has_witness"].as_bool().unwrap());
}
