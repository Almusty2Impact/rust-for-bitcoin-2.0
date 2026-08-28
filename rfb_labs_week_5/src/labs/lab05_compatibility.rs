//! Lab 05 — reason about sender support across address generations.

use crate::model::{AddressFormat, CompatibilityReport, SenderCapabilities};

/// Return whether the sender can decode and pay the selected address format.
pub fn can_send_to(capabilities: SenderCapabilities, format: AddressFormat) -> bool {
    match format {
        AddressFormat::P2pkh => capabilities.base58_p2pkh,
        AddressFormat::P2sh => capabilities.base58_p2sh,
        AddressFormat::P2wpkh => capabilities.bech32,
        AddressFormat::P2tr => capabilities.bech32m,
        AddressFormat::Unknown => false,
    }
}

/// Summarize support for legacy, wrapped SegWit, native SegWit, and Taproot.
pub fn compatibility_report(capabilities: SenderCapabilities) -> CompatibilityReport {
    CompatibilityReport {
        p2pkh: can_send_to(capabilities, AddressFormat::P2pkh),
        p2sh_p2wpkh: can_send_to(capabilities, AddressFormat::P2sh),
        p2wpkh: can_send_to(capabilities, AddressFormat::P2wpkh),
        p2tr: can_send_to(capabilities, AddressFormat::P2tr),
    }
}

/// Choose the best supported single-key receiving format.
pub fn best_supported_format(capabilities: SenderCapabilities) -> Option<AddressFormat> {
    if can_send_to(capabilities, AddressFormat::P2tr) {
        Some(AddressFormat::P2tr)
    } else if can_send_to(capabilities, AddressFormat::P2wpkh) {
        Some(AddressFormat::P2wpkh)
    } else if can_send_to(capabilities, AddressFormat::P2sh) {
        Some(AddressFormat::P2sh)
    } else if can_send_to(capabilities, AddressFormat::P2pkh) {
        Some(AddressFormat::P2pkh)
    } else {
        None
    }
}

/// Explain the encoding requirement that controls sender compatibility.
pub fn required_encoding(format: AddressFormat) -> &'static str {
    match format {
        AddressFormat::P2pkh => "Base58Check",
        AddressFormat::P2sh => "Base58Check",
        AddressFormat::P2wpkh => "Bech32",
        AddressFormat::P2tr => "Bech32m",
        AddressFormat::Unknown => "Unknown",
    }
}
