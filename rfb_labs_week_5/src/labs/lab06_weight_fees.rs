//! Lab 06 — calculate transaction weight, virtual size, and fees.

use crate::model::FeeComparison;
use crate::{LabError, LabResult};

/// Calculate BIP141 weight from stripped and total serialized sizes.
pub fn transaction_weight(stripped_size: u64, total_size: u64) -> LabResult<u64> {
    if total_size < stripped_size {
        return Err(LabError::InvalidSize(
            "total_size cannot be less than stripped_size".to_string(),
        ));
    }
    let witness_size = total_size
        .checked_sub(stripped_size)
        .ok_or_else(|| LabError::InvalidSize("overflow in subtraction".to_string()))?;
    let weight = stripped_size
        .checked_mul(3)
        .and_then(|v| v.checked_add(total_size))
        .ok_or_else(|| LabError::InvalidSize("weight calculation overflow".to_string()))?;
    // Verify: weight = stripped * 3 + total = stripped * 3 + stripped + witness = stripped * 4 + witness
    // So weight = stripped * 4 + witness, meaning witness is non-negative (guaranteed above)
    let _ = witness_size; // used conceptually
    Ok(weight)
}

/// Calculate virtual size as `ceil(weight / 4)`.
pub fn virtual_size(weight: u64) -> u64 {
    weight.div_ceil(4)
}

/// Calculate a fee from virtual size and satoshis per virtual byte.
pub fn fee_sats(vbytes: u64, feerate_sat_vb: u64) -> LabResult<u64> {
    vbytes
        .checked_mul(feerate_sat_vb)
        .ok_or_else(|| LabError::InvalidSize("fee calculation overflow".to_string()))
}

/// Compare illustrative legacy and native-SegWit transactions at one feerate.
pub fn compare_fees(
    legacy_vbytes: u64,
    segwit_vbytes: u64,
    feerate_sat_vb: u64,
) -> LabResult<FeeComparison> {
    let legacy_fee_sats = fee_sats(legacy_vbytes, feerate_sat_vb)?;
    let segwit_fee_sats = fee_sats(segwit_vbytes, feerate_sat_vb)?;
    let savings_sats = legacy_fee_sats
        .checked_sub(segwit_fee_sats)
        .ok_or_else(|| LabError::InvalidSize("savings underflow".to_string()))?;
    Ok(FeeComparison {
        legacy_vbytes,
        segwit_vbytes,
        legacy_fee_sats,
        segwit_fee_sats,
        savings_sats,
    })
}
