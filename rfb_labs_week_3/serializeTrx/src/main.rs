use std::{error::Error, fs, path::PathBuf};

use clap::Parser;
use serde::Deserialize;

// ─── CLI ────────────────────────────────────────────────────────────────

/// Serialize a Bitcoin transaction from a JSON description.
///
/// All transaction data (inputs, outputs, witness) is read from a JSON file.
/// The program validates every hex field and produces the serialized
/// transaction in hexadecimal, plus its byte size.
#[derive(Parser, Debug)]
#[command(name = "serializeTrx", version, about)]
struct Cli {
    /// Path to a JSON file describing the transaction.
    #[arg(value_name = "FILE")]
    tx_file: PathBuf,
}

// ─── JSON input types ───────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct TxFile {
    /// Transaction version (default: 2).
    #[serde(default = "default_version")]
    version: i32,
    /// Whether this is a SegWit transaction (default: false).
    #[serde(default)]
    segwit: bool,
    /// Transaction inputs.
    inputs: Vec<InputFile>,
    /// Transaction outputs.
    outputs: Vec<OutputFile>,
    /// nLocktime (default: 0).
    #[serde(default)]
    locktime: u32,
}

fn default_version() -> i32 {
    2
}

#[derive(Debug, Deserialize)]
struct InputFile {
    /// Previous transaction ID as a 64-character hex string.
    prev_txid: String,
    /// Previous output index (vout).
    vout: u32,
    /// ScriptSig as a hex string (empty string for native SegWit).
    #[serde(default)]
    script_sig: String,
    /// Sequence number, either a u32 or a hex string like "ffffffff".
    #[serde(default = "default_sequence")]
    sequence: String,
    /// Witness items as an array of hex strings (only used when segwit is true).
    #[serde(default)]
    witness: Vec<String>,
}

fn default_sequence() -> String {
    "ffffffff".to_string()
}

#[derive(Debug, Deserialize)]
struct OutputFile {
    /// Value in satoshis.
    value: u64,
    /// ScriptPubKey as a hex string.
    script_pubkey: String,
}

// ─── Internal types (kept from original) ───────────────────────────────

#[derive(Debug)]
struct TxInput {
    prev_txid: Vec<u8>,
    vout: u32,
    script_sig: Vec<u8>,
    sequence: u32,
    witness: Vec<Vec<u8>>,
}

#[derive(Debug)]
struct TxOutput {
    value: u64,
    script_pubkey: Vec<u8>,
}

#[derive(Debug)]
struct Transaction {
    version: i32,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    locktime: u32,
    segwit: bool,
}

// ─── Validation helpers ────────────────────────────────────────────────

/// Parse a hex string into bytes, validating it is valid hexadecimal.
fn validate_hex(hex: &str, field_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    if hex.is_empty() {
        return Ok(vec![]);
    }
    if !hex.len().is_multiple_of(2) {
        return Err(format!(
            "{field_name}: hex string has odd length ({} chars)",
            hex.len()
        )
        .into());
    }
    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        let bad: String = hex.chars().filter(|c| !c.is_ascii_hexdigit()).collect();
        return Err(format!("{field_name}: contains invalid hex characters: {bad:?}").into());
    }
    Ok(hex::decode(hex)?)
}

/// Parse a sequence field which can be either a decimal u32 or a hex string.
fn parse_sequence(raw: &str) -> Result<u32, Box<dyn Error>> {
    if raw.starts_with("0x")
        || raw.starts_with("0X")
        || raw.len() == 8 && raw.chars().all(|c| c.is_ascii_hexdigit())
    {
        let val = u32::from_str_radix(raw.trim_start_matches("0x").trim_start_matches("0X"), 16)?;
        Ok(val)
    } else {
        Ok(raw.parse::<u32>()?)
    }
}

/// Validate that a prev_txid is exactly 32 bytes (64 hex chars).
fn validate_txid(hex: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    if hex.len() != 64 {
        return Err(format!(
            "prev_txid must be exactly 64 hex characters (32 bytes), got {} chars",
            hex.len()
        )
        .into());
    }
    validate_hex(hex, "prev_txid")
}

// ─── Conversion: JSON file → internal types ────────────────────────────

fn build_transaction(file: TxFile) -> Result<Transaction, Box<dyn Error>> {
    if file.inputs.is_empty() {
        return Err("transaction must have at least one input".into());
    }
    if file.outputs.is_empty() {
        return Err("transaction must have at least one output".into());
    }

    let inputs: Vec<TxInput> = file
        .inputs
        .into_iter()
        .enumerate()
        .map(|(i, inp)| {
            let prev_txid = validate_txid(&inp.prev_txid)?;
            let script_sig = validate_hex(&inp.script_sig, &format!("input[{i}].script_sig"))?;
            let sequence =
                parse_sequence(&inp.sequence).map_err(|e| format!("input[{i}].sequence: {e}"))?;
            let mut witness_items = Vec::new();
            for (j, w) in inp.witness.iter().enumerate() {
                let bytes = validate_hex(w, &format!("input[{i}].witness[{j}]"))?;
                witness_items.push(bytes);
            }
            Ok(TxInput {
                prev_txid,
                vout: inp.vout,
                script_sig,
                sequence,
                witness: witness_items,
            })
        })
        .collect::<Result<Vec<TxInput>, Box<dyn Error>>>()?;

    let outputs: Vec<TxOutput> = file
        .outputs
        .into_iter()
        .enumerate()
        .map(|(i, out)| {
            let script_pubkey =
                validate_hex(&out.script_pubkey, &format!("output[{i}].script_pubkey"))?;
            Ok(TxOutput {
                value: out.value,
                script_pubkey,
            })
        })
        .collect::<Result<Vec<TxOutput>, Box<dyn Error>>>()?;

    Ok(Transaction {
        version: file.version,
        inputs,
        outputs,
        locktime: file.locktime,
        segwit: file.segwit,
    })
}

// ─── Serialization (preserved from original) ────────────────────────────

// ┌──────────────────────────────┐
// │ Version          4 bytes     │
// ├──────────────────────────────┤
// │ Marker           1 byte      │
// │ Flag             1 byte      │
// ├──────────────────────────────┤
// │ Input count      VarInt      │
// │ Inputs           Variable    │
// ├──────────────────────────────┤
// │ Output count     VarInt      │
// │ Outputs          Variable    │
// ├──────────────────────────────┤
// │ Witness          Variable    │
// ├──────────────────────────────┤
// │ Locktime         4 bytes  ←  │
// └──────────────────────────────┘

fn serialize_transaction(trx: &Transaction) -> Vec<u8> {
    let mut result = Vec::new();

    // Version
    result.extend_from_slice(&trx.version.to_le_bytes());

    // SegWit marker + flag
    if trx.segwit {
        result.push(0x00); // marker
        result.push(0x01); // flag
    }

    // Input count
    result.extend_from_slice(&encode_varint(trx.inputs.len()));

    // Inputs
    for input in &trx.inputs {
        result.extend_from_slice(&input.prev_txid);
        result.extend_from_slice(&input.vout.to_le_bytes());
        result.extend_from_slice(&encode_varint(input.script_sig.len()));
        result.extend_from_slice(&input.script_sig);
        result.extend_from_slice(&input.sequence.to_le_bytes());
    }

    // Output count
    result.extend_from_slice(&encode_varint(trx.outputs.len()));

    // Outputs
    for output in &trx.outputs {
        result.extend_from_slice(&output.value.to_le_bytes());
        result.extend_from_slice(&encode_varint(output.script_pubkey.len()));
        result.extend_from_slice(&output.script_pubkey);
    }

    // Witness data (one stack per input)
    if trx.segwit {
        for input in &trx.inputs {
            result.extend_from_slice(&encode_varint(input.witness.len()));
            for item in &input.witness {
                result.extend_from_slice(&encode_varint(item.len()));
                result.extend_from_slice(item);
            }
        }
    }

    // Locktime
    result.extend_from_slice(&trx.locktime.to_le_bytes());

    result
}

// Bitcoin CompactSize / VarInt encoding.
//
// Value range              Encoding
// 0 - 252 (0xfc)           1 byte
// 253 - 65,535             FD + 2 bytes LE
// 65,536 - 4,294,967,295   FE + 4 bytes LE
// larger                   FF + 8 bytes LE
fn encode_varint(value: usize) -> Vec<u8> {
    match value {
        0..=0xfc => vec![value as u8],
        0xfd..=0xffff => {
            let mut r = vec![0xfd];
            r.extend_from_slice(&(value as u16).to_le_bytes());
            r
        }
        0x10000..=0xffff_ffff => {
            let mut r = vec![0xfe];
            r.extend_from_slice(&(value as u32).to_le_bytes());
            r
        }
        _ => {
            let mut r = vec![0xff];
            r.extend_from_slice(&(value as u64).to_le_bytes());
            r
        }
    }
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// ─── Main ───────────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Read and parse the JSON file
    let json_str = fs::read_to_string(&cli.tx_file)?;
    let tx_file: TxFile = serde_json::from_str(&json_str)?;

    // Build internal transaction with validation
    let trx = build_transaction(tx_file)?;

    // Serialize
    let serialized = serialize_transaction(&trx);

    println!("Serialized Hex:");
    println!("{}", bytes_to_hex(&serialized));
    println!("\nTransaction size: {} bytes", serialized.len());

    Ok(())
}
