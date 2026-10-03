//! Bitcoin Core RPC client for fetching transactions and previous outputs.

use bitcoin::{Transaction, Txid};
use bitcoincore_rpc::RpcApi;

use crate::error::{ExplorerError, ExplorerResult};
use crate::fees::FeeAnalysis;

/// A thin wrapper around bitcoincore-rpc.
pub struct BitcoinRpc {
    client: bitcoincore_rpc::Client,
}

impl BitcoinRpc {
    /// Connect to Bitcoin Core RPC.
    pub fn connect(url: &str, user: &str, pass: &str) -> ExplorerResult<Self> {
        let client = bitcoincore_rpc::Client::new(
            url,
            bitcoincore_rpc::Auth::UserPass(user.to_owned(), pass.to_owned()),
        )
        .map_err(|e| ExplorerError::RpcError(e.to_string()))?;
        Ok(Self { client })
    }

    /// Test the connection by getting the block count.
    pub fn test_connection(&self) -> ExplorerResult<u64> {
        self.client
            .get_block_count()
            .map_err(|e| ExplorerError::RpcError(e.to_string()))
    }

    /// Fetch a transaction by txid.
    pub fn get_transaction(&self, txid: &Txid) -> ExplorerResult<Transaction> {
        self.client
            .get_raw_transaction(txid, None)
            .map_err(|e| ExplorerError::TransactionNotFound(e.to_string()))
    }

    /// Fetch the value of a previous output (needed for fee calculation).
    pub fn get_prev_output_value(&self, txid: &Txid, vout: u32) -> ExplorerResult<u64> {
        let tx = self.get_transaction(txid)?;
        let out = tx.output.get(vout as usize).ok_or_else(|| {
            ExplorerError::PrevOutputMissing(format!("output {} not found in {}", vout, txid))
        })?;
        Ok(out.value.to_sat())
    }

    /// Calculate the fee for a transaction by fetching all previous outputs.
    pub fn calculate_fee_from_prev_outputs(
        &self,
        tx: &Transaction,
        vsize: usize,
    ) -> ExplorerResult<FeeAnalysis> {
        let is_coinbase = tx.is_coinbase();

        if is_coinbase {
            let output_sats: u64 = tx.output.iter().map(|o| o.value.to_sat()).sum();
            return Ok(FeeAnalysis {
                total_input_sats: 0,
                total_output_sats: output_sats,
                fee_sats: 0,
                fee_rate_sat_vb: 0.0,
                is_coinbase: true,
            });
        }

        let mut total_input_sats: u64 = 0;
        for inp in &tx.input {
            let value =
                self.get_prev_output_value(&inp.previous_output.txid, inp.previous_output.vout)?;
            total_input_sats = total_input_sats.checked_add(value).ok_or_else(|| {
                ExplorerError::PrevOutputMissing("input sum overflow".to_string())
            })?;
        }

        let total_output_sats: u64 = tx.output.iter().map(|o| o.value.to_sat()).sum();
        Ok(crate::fees::calculate_fee(
            total_input_sats,
            total_output_sats,
            vsize,
            is_coinbase,
        ))
    }

    /// Check if a transaction is confirmed and return confirmation count.
    pub fn get_confirmations(&self, txid: &Txid) -> ExplorerResult<u32> {
        let info = self
            .client
            .get_raw_transaction_info(txid, None)
            .map_err(|e| ExplorerError::RpcError(e.to_string()))?;
        Ok(info.confirmations.unwrap_or(0))
    }
}
