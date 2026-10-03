//! Fee calculation: fetch previous outputs and compute fee.

use serde::Serialize;

/// Transaction fee analysis.
#[derive(Debug, Clone, Serialize)]
pub struct FeeAnalysis {
    /// Total input value in satoshis.
    pub total_input_sats: u64,
    /// Total output value in satoshis.
    pub total_output_sats: u64,
    /// Fee in satoshis (inputs - outputs).
    pub fee_sats: u64,
    /// Fee rate in sat/vB.
    pub fee_rate_sat_vb: f64,
    /// Whether this is a coinbase transaction (fee is meaningless).
    pub is_coinbase: bool,
}

/// Calculate the fee given input and output sums.
pub fn calculate_fee(
    input_sats: u64,
    output_sats: u64,
    vsize: usize,
    is_coinbase: bool,
) -> FeeAnalysis {
    if is_coinbase {
        return FeeAnalysis {
            total_input_sats: input_sats,
            total_output_sats: output_sats,
            fee_sats: 0,
            fee_rate_sat_vb: 0.0,
            is_coinbase: true,
        };
    }

    let fee_sats = input_sats.saturating_sub(output_sats);
    let fee_rate_sat_vb = if vsize > 0 {
        fee_sats as f64 / vsize as f64
    } else {
        0.0
    };

    FeeAnalysis {
        total_input_sats: input_sats,
        total_output_sats: output_sats,
        fee_sats,
        fee_rate_sat_vb,
        is_coinbase: false,
    }
}
