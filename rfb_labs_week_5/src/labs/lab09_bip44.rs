//! Lab 09 — decode BIP44 paths and derive the selected address.

use bip39::Mnemonic;
use bitcoin::bip32::{ChildNumber, DerivationPath, Xpriv, Xpub};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::{Address, Network};

use crate::model::Bip44PathInfo;
use crate::{LabError, LabResult};

/// Parse `m / purpose' / coin' / account' / change / index`.
pub fn decode_bip44_path(path: &str) -> LabResult<Bip44PathInfo> {
    let derivation_path: DerivationPath = path
        .parse()
        .map_err(|e: bitcoin::bip32::Error| LabError::InvalidPath(e.to_string()))?;

    let children = Vec::from(derivation_path);
    if children.len() != 5 {
        return Err(LabError::InvalidPath(
            "BIP44 path must have exactly 5 levels".to_string(),
        ));
    }

    let purpose = match children[0] {
        ChildNumber::Hardened { index } => index,
        _ => {
            return Err(LabError::InvalidPath(
                "purpose must be hardened".to_string(),
            ))
        }
    };
    let coin_type = match children[1] {
        ChildNumber::Hardened { index } => index,
        _ => {
            return Err(LabError::InvalidPath(
                "coin_type must be hardened".to_string(),
            ))
        }
    };
    let account = match children[2] {
        ChildNumber::Hardened { index } => index,
        _ => {
            return Err(LabError::InvalidPath(
                "account must be hardened".to_string(),
            ))
        }
    };
    let change = match children[3] {
        ChildNumber::Normal { index } => index,
        _ => return Err(LabError::InvalidPath("change must be normal".to_string())),
    };
    let index = match children[4] {
        ChildNumber::Normal { index } => index,
        _ => {
            return Err(LabError::InvalidPath(
                "address index must be normal".to_string(),
            ))
        }
    };

    Ok(Bip44PathInfo {
        purpose,
        coin_type,
        account,
        change,
        index,
    })
}

/// Translate a decoded path into a concise English explanation.
pub fn describe_bip44_path(info: &Bip44PathInfo) -> String {
    let account_ord = ordinal(info.account + 1);
    let index_ord = ordinal(info.index + 1);
    let chain = if info.change == 0 {
        "receive (external)"
    } else {
        "change (internal)"
    };

    format!(
        "BIP44 path: purpose {} (P2PKH), Bitcoin coin type {}, {} {}, {} branch, {} address",
        info.purpose, info.coin_type, account_ord, "account", chain, index_ord
    )
}

fn ordinal(n: u32) -> &'static str {
    match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        _ => "nth",
    }
}

/// Return the same BIP44 path with only its final address index changed.
pub fn with_address_index(path: &str, new_index: u32) -> LabResult<String> {
    let derivation_path: DerivationPath = path
        .parse()
        .map_err(|e: bitcoin::bip32::Error| LabError::InvalidPath(e.to_string()))?;

    let children = Vec::from(derivation_path);
    if children.len() < 5 {
        return Err(LabError::InvalidPath(
            "path too short for BIP44".to_string(),
        ));
    }

    let mut new_children = children;
    new_children[4] = ChildNumber::Normal { index: new_index };

    let mut result = String::from("m");
    for child in &new_children {
        match child {
            ChildNumber::Hardened { index } => {
                result.push_str(&format!("/{}'", index));
            }
            ChildNumber::Normal { index } => {
                result.push_str(&format!("/{}", index));
            }
        }
    }
    Ok(result)
}

/// Derive the P2PKH address selected by a BIP44 path.
pub fn derive_bip44_address(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    network: Network,
) -> LabResult<String> {
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
    let derived = master
        .derive_priv(&secp, &derivation_path)
        .map_err(|e: bitcoin::bip32::Error| LabError::Derivation(e.to_string()))?;

    let xpub = Xpub::from_priv(&secp, &derived);
    let compressed_pubkey = xpub.to_pub();
    let public_key = bitcoin::PublicKey::from(compressed_pubkey);
    let address = Address::p2pkh(public_key, network);
    Ok(address.to_string())
}
