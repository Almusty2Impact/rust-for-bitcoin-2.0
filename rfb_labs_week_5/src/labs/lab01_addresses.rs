//! Lab 01 — identify Bitcoin address formats and enforce network safety.

use bitcoin::{address::NetworkChecked, Address, Network};

use crate::model::{AddressFormat, AddressReport};
use crate::{LabError, LabResult};

/// Identify an address family from its human-readable prefix.
pub fn identify_prefix(address: &str) -> AddressFormat {
    if address.starts_with('1') {
        AddressFormat::P2pkh
    } else if address.starts_with('3') {
        AddressFormat::P2sh
    } else if address.starts_with("bc1q") {
        AddressFormat::P2wpkh
    } else if address.starts_with("bc1p") {
        AddressFormat::P2tr
    } else if address.starts_with("tb1q") || address.starts_with("bcrt1q") {
        AddressFormat::P2wpkh
    } else if address.starts_with("tb1p") || address.starts_with("bcrt1p") {
        AddressFormat::P2tr
    } else if address.starts_with('2') {
        AddressFormat::P2sh
    } else if address.starts_with('m') || address.starts_with('n') {
        AddressFormat::P2pkh
    } else {
        AddressFormat::Unknown
    }
}

/// Return the expected human-readable prefix for a format on a selected network.
pub fn expected_prefix(format: AddressFormat, network: Network) -> Option<&'static str> {
    match (format, network) {
        (AddressFormat::P2pkh, Network::Bitcoin) => Some("1"),
        (AddressFormat::P2sh, Network::Bitcoin) => Some("3"),
        (AddressFormat::P2wpkh, Network::Bitcoin) => Some("bc1q"),
        (AddressFormat::P2tr, Network::Bitcoin) => Some("bc1p"),
        (AddressFormat::P2pkh, Network::Regtest) => Some("m/n"),
        (AddressFormat::P2sh, Network::Regtest) => Some("2"),
        (AddressFormat::P2wpkh, Network::Regtest) => Some("bcrt1q"),
        (AddressFormat::P2tr, Network::Regtest) => Some("bcrt1p"),
        // Testnet, Signet, Testnet4 all use tb1/t prefixes
        (AddressFormat::P2pkh, _) => Some("m/n"),
        (AddressFormat::P2sh, _) => Some("2"),
        (AddressFormat::P2wpkh, _) => Some("tb1q"),
        (AddressFormat::P2tr, _) => Some("tb1p"),
        (AddressFormat::Unknown, _) => None,
    }
}

/// Parse an address, reject the wrong network, and return its full report.
pub fn inspect_address(address: &str, network: Network) -> LabResult<AddressReport> {
    let unchecked: Address<bitcoin::address::NetworkUnchecked> = address
        .parse()
        .map_err(|e: bitcoin::address::ParseError| LabError::InvalidAddress(e.to_string()))?;

    let parsed: Address<NetworkChecked> = unchecked
        .require_network(network)
        .map_err(|e| LabError::WrongNetwork(e.to_string()))?;

    let format = match parsed.address_type() {
        Some(bitcoin::address::AddressType::P2pkh) => AddressFormat::P2pkh,
        Some(bitcoin::address::AddressType::P2sh) => AddressFormat::P2sh,
        Some(bitcoin::address::AddressType::P2wpkh) => AddressFormat::P2wpkh,
        Some(bitcoin::address::AddressType::P2tr) => AddressFormat::P2tr,
        _ => AddressFormat::Unknown,
    };

    Ok(AddressReport {
        address: address.to_owned(),
        network: format!("{:?}", network).to_lowercase(),
        format,
        script_pubkey_hex: parsed.script_pubkey().to_hex_string(),
    })
}

/// Return the scriptPubKey encoded by a network-checked address.
pub fn script_pubkey_hex(address: &str, network: Network) -> LabResult<String> {
    let unchecked: Address<bitcoin::address::NetworkUnchecked> = address
        .parse()
        .map_err(|e: bitcoin::address::ParseError| LabError::InvalidAddress(e.to_string()))?;

    let parsed = unchecked
        .require_network(network)
        .map_err(|e| LabError::WrongNetwork(e.to_string()))?;

    Ok(parsed.script_pubkey().to_hex_string())
}
