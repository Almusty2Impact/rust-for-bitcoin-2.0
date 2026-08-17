use std::io::Error;

use sha2::{Digest, Sha256};

use transaction::{Amount, Input, Output, Transaction, Txid};

mod transaction;

// Bitcoin uses little-endian encoding for most numeric fields.

/// Read 4 bytes as a little-endian u32 (advances the slice).
fn read_u32(bytes_slice: &mut &[u8]) -> Result<u32, Error> {
    if bytes_slice.len() < 4 {
        return Err(Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "not enough bytes for u32",
        ));
    }
    let (val_bytes, rest) = bytes_slice.split_at(4);
    *bytes_slice = rest;
    Ok(u32::from_le_bytes(val_bytes.try_into().unwrap()))
}

/// Read 8 bytes as a little-endian u64 (advances the slice).
fn read_u64(transaction_bytes: &mut &[u8]) -> u64 {
    let (val_bytes, rest) = transaction_bytes.split_at(8);
    *transaction_bytes = rest;
    u64::from_le_bytes(val_bytes.try_into().unwrap())
}

/// Read 8 bytes as a little-endian u64 and wrap in Amount.
fn read_amount(transaction_bytes: &mut &[u8]) -> Result<Amount, Error> {
    if transaction_bytes.len() < 8 {
        return Err(Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "not enough bytes for amount",
        ));
    }
    let val = read_u64(transaction_bytes);
    Ok(Amount::from_sat(val))
}

/// Read a Bitcoin CompactSize unsigned integer (VarInt).
fn read_compact_size(transaction_bytes: &mut &[u8]) -> Result<u64, Error> {
    if transaction_bytes.is_empty() {
        return Err(Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "not enough bytes for compact size",
        ));
    }
    let first = transaction_bytes[0];
    *transaction_bytes = &transaction_bytes[1..];
    match first {
        0x00..=0xfc => Ok(first as u64),
        0xfd => {
            let (b, rest) = transaction_bytes.split_at(2);
            *transaction_bytes = rest;
            Ok(u16::from_le_bytes(b.try_into().unwrap()) as u64)
        }
        0xfe => {
            let (b, rest) = transaction_bytes.split_at(4);
            *transaction_bytes = rest;
            Ok(u32::from_le_bytes(b.try_into().unwrap()) as u64)
        }
        0xff => {
            let (b, rest) = transaction_bytes.split_at(8);
            *transaction_bytes = rest;
            Ok(u64::from_le_bytes(b.try_into().unwrap()))
        }
    }
}

/// Read 32 bytes as a Txid (advances the slice).
fn read_txid(transaction_bytes: &mut &[u8]) -> Result<Txid, Error> {
    if transaction_bytes.len() < 32 {
        return Err(Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "not enough bytes for txid",
        ));
    }
    let (txid_bytes, rest) = transaction_bytes.split_at(32);
    *transaction_bytes = rest;
    let mut arr = [0u8; 32];
    arr.copy_from_slice(txid_bytes);
    Ok(Txid::from_bytes(arr))
}

/// Read a variable-length script and return it as a Vec<u8>.
fn read_script_size(transaction_bytes: &mut &[u8]) -> Result<Vec<u8>, Error> {
    let len = read_compact_size(transaction_bytes)? as usize;
    if transaction_bytes.len() < len {
        return Err(Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "not enough bytes for script",
        ));
    }
    let (script, rest) = transaction_bytes.split_at(len);
    *transaction_bytes = rest;
    Ok(script.to_vec())
}

/// Read 4 bytes as version (alias for read_u32).
fn read_version_byte(transaction_bytes: &mut &[u8]) -> Result<u32, Error> {
    read_u32(transaction_bytes)
}

/// Double-SHA256 the raw transaction bytes (without witness) to compute the txid.
fn hash_raw_transaction(raw_bytes: &[u8]) -> Result<Txid, Error> {
    let hash1 = Sha256::digest(raw_bytes);
    let hash2 = Sha256::digest(hash1);
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&hash2);
    Ok(Txid::from_bytes(arr))
}

/// Decode a raw hex-encoded Bitcoin transaction (SegWit-aware) into a JSON string.
pub fn decode_transaction(transaction_hex: String) -> Result<String, Box<dyn std::error::Error>> {
    let raw_bytes = hex::decode(&transaction_hex)?;
    let mut bytes: &[u8] = &raw_bytes;

    // Version (4 bytes LE)
    let version = read_version_byte(&mut bytes)?;

    // Check for SegWit marker (0x00) and flag (0x01)
    let is_segwit = bytes.len() >= 2 && bytes[0] == 0x00 && bytes[1] == 0x01;
    if is_segwit {
        // Skip marker + flag
        bytes = &bytes[2..];
    }

    // Input count
    let input_count = read_compact_size(&mut bytes)? as usize;
    let mut inputs = Vec::with_capacity(input_count);

    for _ in 0..input_count {
        let txid = read_txid(&mut bytes)?;
        let output_index = read_u32(&mut bytes)?;
        let script_sig = read_script_size(&mut bytes)?;
        let sequence = read_u32(&mut bytes)?;
        inputs.push(Input {
            txid,
            output_index,
            script_sig,
            sequence,
        });
    }

    // Output count
    let output_count = read_compact_size(&mut bytes)? as usize;
    let mut outputs = Vec::with_capacity(output_count);

    for _ in 0..output_count {
        let amount = read_amount(&mut bytes)?;
        let script_pubkey = read_script_size(&mut bytes)?;
        outputs.push(Output {
            amount,
            script_pubkey,
        });
    }

    // Skip witness data if present
    if is_segwit {
        for _ in 0..input_count {
            let witness_items = read_compact_size(&mut bytes)? as usize;
            for _ in 0..witness_items {
                let item_len = read_compact_size(&mut bytes)? as usize;
                if bytes.len() < item_len {
                    return Err("not enough bytes for witness item".into());
                }
                bytes = &bytes[item_len..];
            }
        }
    }

    // Locktime (4 bytes LE)
    let lock_time = read_u32(&mut bytes)?;

    // Compute txid from the non-witness serialization
    let mut no_witness = Vec::new();
    no_witness.extend_from_slice(&version.to_le_bytes());
    no_witness.extend_from_slice(&encode_varint(input_count as u64));

    for input in &inputs {
        no_witness.extend_from_slice(&input.txid.0);
        no_witness.extend_from_slice(&input.output_index.to_le_bytes());
        no_witness.extend_from_slice(&encode_varint(input.script_sig.len() as u64));
        no_witness.extend_from_slice(&input.script_sig);
        no_witness.extend_from_slice(&input.sequence.to_le_bytes());
    }

    no_witness.extend_from_slice(&encode_varint(output_count as u64));
    for output in &outputs {
        no_witness.extend_from_slice(&output.amount.0.to_le_bytes());
        no_witness.extend_from_slice(&encode_varint(output.script_pubkey.len() as u64));
        no_witness.extend_from_slice(&output.script_pubkey);
    }

    no_witness.extend_from_slice(&lock_time.to_le_bytes());

    let transaction_id = hash_raw_transaction(&no_witness)?;

    let trx = Transaction {
        transaction_id,
        version,
        inputs,
        outputs,
        lock_time,
    };

    let json = serde_json::to_string_pretty(&trx)?;
    Ok(json)
}

/// Encode a value as a Bitcoin CompactSize (VarInt).
fn encode_varint(value: u64) -> Vec<u8> {
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
            r.extend_from_slice(&value.to_le_bytes());
            r
        }
    }
}
