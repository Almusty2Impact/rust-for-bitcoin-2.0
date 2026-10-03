//! Identify script types and derive addresses from scripts.

use bitcoin::{Address, Network, ScriptBuf};

use serde::Serialize;

/// Recognised Bitcoin script type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScriptType {
    P2pkh,
    P2sh,
    P2wpkh,
    P2wsh,
    P2tr,
    OpReturn,
    P2pk,
    Multisig,
    Unknown,
}

/// Result of inspecting a script.
#[derive(Debug, Clone, Serialize)]
pub struct ScriptInfo {
    /// The classified script type.
    pub script_type: ScriptType,
    /// The derived address, if any.
    pub address: Option<String>,
    /// The network assumed for address derivation.
    pub network: String,
    /// Whether this is a witness program.
    pub is_witness: bool,
}

/// Identify the script type from a scriptPubKey.
pub fn identify_script_type(script: &ScriptBuf) -> ScriptType {
    if script.is_p2pkh() {
        ScriptType::P2pkh
    } else if script.is_p2sh() {
        ScriptType::P2sh
    } else if script.is_p2wpkh() {
        ScriptType::P2wpkh
    } else if script.is_p2wsh() {
        ScriptType::P2wsh
    } else if script.is_p2tr() {
        ScriptType::P2tr
    } else if script.is_op_return() {
        ScriptType::OpReturn
    } else if script.is_p2pk() {
        ScriptType::P2pk
    } else {
        ScriptType::Unknown
    }
}

/// Derive an address from a scriptPubKey on the given network.
pub fn address_from_script(script: &ScriptBuf, network: Network) -> Option<Address> {
    Address::from_script(script, network).ok()
}

/// Full inspection of a script: type, address, witness status.
pub fn inspect_script(script_hex: &str, network: Network) -> crate::ExplorerResult<ScriptInfo> {
    let bytes =
        hex::decode(script_hex).map_err(|e| crate::ExplorerError::InvalidHex(e.to_string()))?;
    let script = ScriptBuf::from_bytes(bytes);

    let script_type = identify_script_type(&script);
    let address = address_from_script(&script, network).map(|a| a.to_string());

    let is_witness = matches!(
        script_type,
        ScriptType::P2wpkh | ScriptType::P2wsh | ScriptType::P2tr
    );

    Ok(ScriptInfo {
        script_type,
        address,
        network: format!("{:?}", network).to_lowercase(),
        is_witness,
    })
}

/// Derive script type and address for each output in a decoded transaction.
pub fn classify_outputs(output_scripts: &[String], network: Network) -> Vec<ScriptInfo> {
    output_scripts
        .iter()
        .map(|hex| {
            inspect_script(hex, network).unwrap_or_else(|_| ScriptInfo {
                script_type: ScriptType::Unknown,
                address: None,
                network: format!("{:?}", network).to_lowercase(),
                is_witness: false,
            })
        })
        .collect()
}
