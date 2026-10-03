//! Lab 10 — prove deterministic recovery across BIP44, BIP49, and BIP84.

use bip39::Mnemonic;
use bitcoin::bip32::{DerivationPath, Xpriv, Xpub};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::{Address, Network};

use crate::model::{AddressFormat, DerivedAddressSet};
use crate::{LabError, LabResult};

/// Derive one address from an arbitrary full path and selected script family.
pub fn derive_address_for_path(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    format: AddressFormat,
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
    let compressed = xpub.to_pub();

    let address = match format {
        AddressFormat::P2pkh => {
            let public_key = bitcoin::PublicKey::from(compressed);
            Address::p2pkh(public_key, network)
        }
        AddressFormat::P2sh => {
            // Wrapped SegWit (P2SH-P2WPKH)
            let wpkh_script = Address::p2wpkh(&compressed, network).script_pubkey();
            Address::p2sh(&wpkh_script, network)
                .map_err(|e: bitcoin::address::P2shError| LabError::InvalidAddress(e.to_string()))?
        }
        AddressFormat::P2wpkh => Address::p2wpkh(&compressed, network),
        _ => {
            return Err(LabError::InvalidScript(format!(
                "unsupported format: {:?}",
                format
            )))
        }
    };

    Ok(address.to_string())
}

/// Derive index `n` on the BIP44, BIP49, and BIP84 receive branches.
pub fn derive_address_set(
    mnemonic: &str,
    passphrase: &str,
    account: u32,
    index: u32,
    network: Network,
) -> LabResult<DerivedAddressSet> {
    let bip44_path = format!("m/44'/{}'/{}'/0/{}", 1, account, index);
    let bip49_path = format!("m/49'/{}'/{}'/0/{}", 1, account, index);
    let bip84_path = format!("m/84'/{}'/{}'/0/{}", 1, account, index);

    let bip44_p2pkh = derive_address_for_path(
        mnemonic,
        passphrase,
        &bip44_path,
        AddressFormat::P2pkh,
        network,
    )?;
    let bip49_p2sh_p2wpkh = derive_address_for_path(
        mnemonic,
        passphrase,
        &bip49_path,
        AddressFormat::P2sh,
        network,
    )?;
    let bip84_p2wpkh = derive_address_for_path(
        mnemonic,
        passphrase,
        &bip84_path,
        AddressFormat::P2wpkh,
        network,
    )?;

    Ok(DerivedAddressSet {
        bip44_p2pkh,
        bip49_p2sh_p2wpkh,
        bip84_p2wpkh,
    })
}

/// Prove that identical mnemonic, passphrase, path, and network reproduce an address.
pub fn recovery_is_repeatable(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    format: AddressFormat,
    network: Network,
) -> LabResult<bool> {
    let first = derive_address_for_path(mnemonic, passphrase, path, format, network)?;
    let second = derive_address_for_path(mnemonic, passphrase, path, format, network)?;
    Ok(first == second)
}

/// Prove that changing only the final index selects a different address.
pub fn changing_index_changes_address(
    mnemonic: &str,
    passphrase: &str,
    first_path: &str,
    second_path: &str,
    format: AddressFormat,
    network: Network,
) -> LabResult<bool> {
    let first = derive_address_for_path(mnemonic, passphrase, first_path, format, network)?;
    let second = derive_address_for_path(mnemonic, passphrase, second_path, format, network)?;
    Ok(first != second)
}
