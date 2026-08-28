//! Lab 08 — derive BIP32 extended private and public keys.

use bip39::Mnemonic;
use bitcoin::bip32::{ChildNumber, DerivationPath, Xpriv, Xpub};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Network;

use crate::model::ExtendedKeyReport;
use crate::{LabError, LabResult};

/// Create the master extended private key from a BIP39 recovery setup.
pub fn master_xpriv(mnemonic: &str, passphrase: &str, network: Network) -> LabResult<String> {
    let parsed: Mnemonic = mnemonic
        .parse()
        .map_err(|e: bip39::Error| LabError::InvalidMnemonic(e.to_string()))?;
    let seed = parsed.to_seed(passphrase);
    let xpriv = Xpriv::new_master(network, &seed)
        .map_err(|e: bitcoin::bip32::Error| LabError::Derivation(e.to_string()))?;
    Ok(xpriv.to_string())
}

/// Derive an extended private/public key pair at a complete path.
pub fn derive_extended_keys(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    network: Network,
) -> LabResult<ExtendedKeyReport> {
    let parsed: Mnemonic = mnemonic
        .parse()
        .map_err(|e: bip39::Error| LabError::InvalidMnemonic(e.to_string()))?;
    let seed = parsed.to_seed(passphrase);
    let master = Xpriv::new_master(network, &seed)
        .map_err(|e: bitcoin::bip32::Error| LabError::Derivation(e.to_string()))?;

    let derivation_path: DerivationPath = path
        .parse()
        .map_err(|e: bitcoin::bip32::Error| LabError::InvalidPath(e.to_string()))?;

    let secp = Secp256k1::new();
    let derived_xpriv = master
        .derive_priv(&secp, &derivation_path)
        .map_err(|e: bitcoin::bip32::Error| LabError::Derivation(e.to_string()))?;
    let derived_xpub = Xpub::from_priv(&secp, &derived_xpriv);

    Ok(ExtendedKeyReport {
        derivation_path: path.to_owned(),
        xpriv: derived_xpriv.to_string(),
        xpub: derived_xpub.to_string(),
    })
}

/// Derive a normal public child from an xpub without private key material.
pub fn derive_normal_child_xpub(parent_xpub: &str, index: u32) -> LabResult<String> {
    let xpub: Xpub = parent_xpub
        .parse()
        .map_err(|e: bitcoin::bip32::Error| LabError::InvalidKey(e.to_string()))?;
    let secp = Secp256k1::new();
    let path = [ChildNumber::Normal { index }];
    let child = xpub
        .derive_pub(&secp, &path)
        .map_err(|e: bitcoin::bip32::Error| LabError::Derivation(e.to_string()))?;
    Ok(child.to_string())
}

/// Return whether a textual path contains at least one hardened step.
pub fn path_contains_hardened_step(path: &str) -> LabResult<bool> {
    let derivation_path: DerivationPath = path
        .parse()
        .map_err(|e: bitcoin::bip32::Error| LabError::InvalidPath(e.to_string()))?;

    let children: Vec<ChildNumber> = Vec::from(derivation_path);
    Ok(children
        .iter()
        .any(|child| matches!(child, ChildNumber::Hardened { .. })))
}
