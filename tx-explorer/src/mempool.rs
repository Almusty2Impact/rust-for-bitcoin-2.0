//! Fee + confirmation lookup via mempool.space (Esplora API).
//! Fallback for when no Bitcoin Core node is available.

use std::time::Duration;
use serde::Deserialize;
use crate::error::{ExplorerError, ExplorerResult};
use crate::fees::FeeAnalysis;

const BASE: &str = "https://mempool.space/api";

#[derive(Deserialize)]
struct MempoolTx {
    fee: u64,
    weight: u64,
    vout: Vec<MempoolVout>,
    status: MempoolStatus,
}

#[derive(Deserialize)]
struct MempoolVout {
    value: u64,
}

#[derive(Deserialize)]
struct MempoolStatus {
    confirmed: bool,
    block_height: Option<u64>,
}

pub struct MempoolSpace {
    base: String,
}

impl MempoolSpace {
    pub fn new() -> Self {
        Self { base: BASE.into() }
    }

    /// Fetch raw tx hex from the API.
    pub fn raw_hex(&self, txid: &str) -> ExplorerResult<String> {
    let url = format!("{}/tx/{}/hex", self.base, txid);
    let body = ureq::get(&url)
        .timeout(Duration::from_secs(15))
        .call()
        .map_err(|e| ExplorerError::RpcError(e.to_string()))?
        .into_string()
        .map_err(|e| ExplorerError::RpcError(e.to_string()))?;
    Ok(body.trim().to_string())
}

    /// Fetch the tx JSON once and reuse for fee + confirmations.
    fn fetch_tx(&self, txid: &str) -> ExplorerResult<MempoolTx> {
    let url = format!("{}/tx/{}", self.base, txid);
    let body = ureq::get(&url)
        .timeout(Duration::from_secs(15))
        .call()
        .map_err(|e| ExplorerError::RpcError(e.to_string()))?
        .into_string()
        .map_err(|e| ExplorerError::RpcError(e.to_string()))?;
    serde_json::from_str(&body).map_err(|e| ExplorerError::RpcError(e.to_string()))
}

    /// Fetch fee analysis (fee, vsize, fee rate, input/output sums).
    pub fn fetch_fee(&self, txid: &str) -> ExplorerResult<FeeAnalysis> {
        let tx = self.fetch_tx(txid)?;
        let vsize = tx.weight.div_ceil(4) as usize;
        let total_output_sats: u64 = tx.vout.iter().map(|v| v.value).sum();
        // inputs = outputs + fee, so the arithmetic is self-consistent
        let total_input_sats = total_output_sats + tx.fee;
        let fee_rate = tx.fee as f64 / vsize.max(1) as f64;

        Ok(FeeAnalysis {
            total_input_sats,
            total_output_sats,
            fee_sats: tx.fee,
            fee_rate_sat_vb: fee_rate,
            is_coinbase: false,
        })
    }

    /// Fetch confirmation count (0 = unconfirmed).
    pub fn confirmations(&self, txid: &str) -> ExplorerResult<u32> {
        let tx = self.fetch_tx(txid)?;
        if !tx.status.confirmed {
            return Ok(0);
        }
        let Some(height) = tx.status.block_height else {
            return Ok(0);
        };

        // /blocks/tip/height returns a plain number, not JSON
        let tip_body = ureq::get(&format!("{}/blocks/tip/height", self.base))
            .timeout(Duration::from_secs(15))
            .call()
            .map_err(|e| ExplorerError::RpcError(e.to_string()))?
            .into_string()
            .map_err(|e| ExplorerError::RpcError(e.to_string()))?
            .trim()
            .to_string();
        let tip: u64 = tip_body
            .parse()
            .map_err(|e: std::num::ParseIntError| ExplorerError::RpcError(e.to_string()))?;

        Ok((tip - height + 1) as u32)
    }
}