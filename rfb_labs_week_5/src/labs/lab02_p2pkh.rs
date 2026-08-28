//! Lab 02 — construct and explain legacy P2PKH.

use bitcoin::{Address, Network, PublicKey, ScriptBuf};

use crate::model::P2pkhSpendTemplate;
use crate::{LabError, LabResult};

/// Derive a P2PKH address from a serialized public key.
pub fn derive_p2pkh_address(public_key_hex: &str, network: Network) -> LabResult<String> {
    let public_key: PublicKey = public_key_hex
        .parse()
        .map_err(|e: bitcoin::key::ParsePublicKeyError| LabError::InvalidKey(e.to_string()))?;
    Ok(Address::p2pkh(public_key, network).to_string())
}

/// Build the P2PKH scriptPubKey for the serialized public key.
pub fn build_p2pkh_script_pubkey(public_key_hex: &str) -> LabResult<String> {
    let public_key: PublicKey = public_key_hex
        .parse()
        .map_err(|e: bitcoin::key::ParsePublicKeyError| LabError::InvalidKey(e.to_string()))?;
    let script = ScriptBuf::new_p2pkh(&public_key.pubkey_hash());
    Ok(script.to_hex_string())
}

/// Return the HASH160 commitment made to the public key.
pub fn committed_pubkey_hash(public_key_hex: &str) -> LabResult<String> {
    let public_key: PublicKey = public_key_hex
        .parse()
        .map_err(|e: bitcoin::key::ParsePublicKeyError| LabError::InvalidKey(e.to_string()))?;
    Ok(public_key.pubkey_hash().to_string())
}

/// Place a signature and public key in the legacy unlocking location.
pub fn p2pkh_spend_template(
    signature_hex: &str,
    public_key_hex: &str,
) -> LabResult<P2pkhSpendTemplate> {
    let _public_key: PublicKey = public_key_hex
        .parse()
        .map_err(|e: bitcoin::key::ParsePublicKeyError| LabError::InvalidKey(e.to_string()))?;

    Ok(P2pkhSpendTemplate {
        script_sig_items: vec![signature_hex.to_owned(), public_key_hex.to_owned()],
        witness_items: vec![],
    })
}
