//! Lab 03 — wrap a 2-of-3 multisig rule in P2SH.

use bitcoin::opcodes::all::OP_CHECKMULTISIG;
use bitcoin::script::Builder;
use bitcoin::{Address, Network, PublicKey, ScriptBuf};

use crate::model::P2shReport;
use crate::{LabError, LabResult};

/// Build `2 <pub1> <pub2> <pub3> 3 OP_CHECKMULTISIG`.
pub fn build_2_of_3_redeem_script(public_keys: [&str; 3]) -> LabResult<String> {
    let keys: Result<Vec<PublicKey>, LabError> = public_keys
        .iter()
        .map(|k| {
            k.parse()
                .map_err(|e: bitcoin::key::ParsePublicKeyError| LabError::InvalidKey(e.to_string()))
        })
        .collect();
    let keys = keys?;

    let script = Builder::new()
        .push_int(2)
        .push_key(&keys[0])
        .push_key(&keys[1])
        .push_key(&keys[2])
        .push_int(3)
        .push_opcode(OP_CHECKMULTISIG)
        .into_script();

    Ok(script.to_hex_string())
}

/// Derive the P2SH address that commits to a redeemScript.
pub fn derive_p2sh_address(redeem_script_hex: &str, network: Network) -> LabResult<String> {
    let bytes = hex::decode(redeem_script_hex)
        .map_err(|e: hex::FromHexError| LabError::InvalidScript(e.to_string()))?;
    let script = ScriptBuf::from_bytes(bytes);
    let address = Address::p2sh(&script, network)
        .map_err(|e: bitcoin::address::P2shError| LabError::InvalidAddress(e.to_string()))?;
    Ok(address.to_string())
}

/// Return the outer `OP_HASH160 <scriptHash> OP_EQUAL` scriptPubKey.
pub fn build_p2sh_script_pubkey(redeem_script_hex: &str) -> LabResult<String> {
    let bytes = hex::decode(redeem_script_hex)
        .map_err(|e: hex::FromHexError| LabError::InvalidScript(e.to_string()))?;
    let script = ScriptBuf::from_bytes(bytes);
    let address = Address::p2sh(&script, Network::Regtest)
        .map_err(|e: bitcoin::address::P2shError| LabError::InvalidAddress(e.to_string()))?;
    Ok(address.script_pubkey().to_hex_string())
}

/// Collect the inner script, outer address, and scriptPubKey in one report.
pub fn inspect_p2sh_multisig(public_keys: [&str; 3], network: Network) -> LabResult<P2shReport> {
    let redeem_script_hex = build_2_of_3_redeem_script(public_keys)?;
    let bytes = hex::decode(&redeem_script_hex)
        .map_err(|e: hex::FromHexError| LabError::InvalidScript(e.to_string()))?;
    let script = ScriptBuf::from_bytes(bytes);
    let address = Address::p2sh(&script, network)
        .map_err(|e: bitcoin::address::P2shError| LabError::InvalidAddress(e.to_string()))?;
    let script_pubkey_hex = address.script_pubkey().to_hex_string();

    Ok(P2shReport {
        redeem_script_hex,
        address: address.to_string(),
        script_pubkey_hex,
    })
}
