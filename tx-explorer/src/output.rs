//! Output formatting: human-readable tables with colors, and JSON.

use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};
use owo_colors::OwoColorize;
use serde::Serialize;

use crate::analysis::TransactionAnalysis;
use crate::decode::DecodedTransaction;
use crate::fees::FeeAnalysis;
use crate::script::ScriptInfo;

/// The complete exploration result ready for output.
#[derive(Debug, Clone, Serialize)]
pub struct ExplorationResult {
    /// Decoded transaction fields.
    pub decoded: DecodedTransaction,
    /// Script classification for each output.
    pub script_info: Vec<ScriptInfo>,
    /// Fee analysis (may be absent if prev outputs unavailable).
    pub fee: Option<FeeAnalysis>,
    /// Transaction analysis summary.
    pub analysis: TransactionAnalysis,
    /// Confirmation count (None if offline).
    pub confirmations: Option<u32>,
}

impl ExplorationResult {
    /// Render as JSON string.
    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Render as human-readable coloured terminal output.
    pub fn to_terminal(&self) -> String {
        let mut out = String::new();

        // ── Header ──
        out.push_str(&format!(
            "\n{} {}\n",
            "⚡ Transaction Explorer".bold().cyan(),
            "──────────────────────".dimmed()
        ));

        // ── Overview ──
        out.push_str(&format!(
            "  {} {}\n",
            "TXID:".bold().yellow(),
            self.decoded.txid.white()
        ));
        if self.decoded.has_witness {
            out.push_str(&format!(
                "  {} {}\n",
                "WTXID:".bold().yellow(),
                self.decoded.wtxid.white()
            ));
        }
        out.push_str(&format!(
            "  {} {}   {} {}   {} {}\n",
            "Version:".bold().yellow(),
            self.decoded.version,
            "Locktime:".bold().yellow(),
            self.decoded.locktime,
            "Coinbase:".bold().yellow(),
            format_bool(self.decoded.is_coinbase),
        ));

        // ── SegWit & RBF ──
        out.push_str(&format!(
            "  {} {}   {} {}\n",
            "SegWit:".bold().yellow(),
            self.analysis.segwit_variant.green(),
            "RBF:".bold().yellow(),
            format_bool(self.analysis.signals_rbf),
        ));

        // ── Confirmations ──
        if let Some(confs) = self.confirmations {
            let conf_str = if confs == 0 {
                "0 (unconfirmed)".red().to_string()
            } else {
                format!("{} confirmed", confs).green().to_string()
            };
            out.push_str(&format!(
                "  {} {}\n",
                "Confirmations:".bold().yellow(),
                conf_str
            ));
        }

        out.push('\n');

        // ── Size / Weight ──
        let mut size_table = Table::new();
        size_table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Metric", "Value"])
            .add_row(vec!["Size (stripped)", &self.decoded.size.to_string()])
            .add_row(vec!["Total size", &self.decoded.total_size.to_string()])
            .add_row(vec!["Weight (WU)", &self.decoded.weight.to_string()])
            .add_row(vec!["Virtual size (vB)", &self.decoded.vsize.to_string()]);

        if self.analysis.witness_discount > 0 {
            size_table.add_row(vec![
                "Witness discount",
                &format!("-{} bytes", self.analysis.witness_discount),
            ]);
        }

        out.push_str(&format!("{}\n\n", size_table));

        // ── Fee ──
        if let Some(ref fee) = self.fee {
            if fee.is_coinbase {
                out.push_str(&format!(
                    "  {} (coinbase — no fee)\n\n",
                    "Fee:".bold().yellow()
                ));
            } else {
                out.push_str(&format!(
                    "  {} {} sats   {} {:.2} sat/vB\n\n",
                    "Fee:".bold().yellow(),
                    fee.fee_sats.green(),
                    "Fee rate:".bold().yellow(),
                    fee.fee_rate_sat_vb,
                ));
                out.push_str(&format!(
                    "  {} {} sats   {} {} sats\n\n",
                    "Inputs sum:".bold().yellow(),
                    fee.total_input_sats,
                    "Outputs sum:".bold().yellow(),
                    fee.total_output_sats,
                ));
            }
        } else {
            out.push_str(&format!(
                "  {}\n\n",
                "Fee: — (requires RPC to fetch previous outputs)".dimmed()
            ));
        }

        // ── Inputs ──
        out.push_str(&format!(
            "{} ({}):\n",
            "📥 Inputs".bold().cyan(),
            self.decoded.inputs.len()
        ));

        let mut input_table = Table::new();
        input_table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                "#",
                "Previous Output",
                "ScriptSig",
                "Witness",
                "Seq",
                "RBF",
            ]);

        for (i, inp) in self.decoded.inputs.iter().enumerate() {
            let script_sig = if inp.script_sig_hex.is_empty() {
                "∅".dimmed().to_string()
            } else {
                truncate_str(&inp.script_sig_asm, 30)
            };
            let witness = if inp.witness.is_empty() {
                "∅".dimmed().to_string()
            } else {
                format!("{} items", inp.witness.len()).green().to_string()
            };
            let rbf = if inp.signals_rbf {
                "✓".green().to_string()
            } else {
                "✗".dimmed().to_string()
            };
            input_table.add_row(vec![
                &i.to_string(),
                &format!("{}:{}", inp.prev_txid, inp.prev_vout),
                &script_sig,
                &witness,
                &format!("0x{:08x}", inp.sequence),
                &rbf,
            ]);
        }
        out.push_str(&format!("{}\n\n", input_table));

        // ── Outputs ──
        out.push_str(&format!(
            "{} ({}):\n",
            "📤 Outputs".bold().cyan(),
            self.decoded.outputs.len()
        ));

        let mut output_table = Table::new();
        output_table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                "#",
                "Value (BTC)",
                "Type",
                "Address",
                "ScriptPubKey (ASM)",
            ]);

        for (i, out) in self.decoded.outputs.iter().enumerate() {
            let info = self.script_info.get(i);
            let type_str = info
                .map(|s| format!("{:?}", s.script_type))
                .unwrap_or_else(|| "UNKNOWN".to_string());
            let addr_str = info
                .and_then(|s| s.address.as_ref())
                .cloned()
                .unwrap_or_else(|| "—".to_string());
            let asm = truncate_str(&out.script_pubkey_asm, 40);
            output_table.add_row(vec![
                &out.index.to_string(),
                &out.value_btc,
                &type_str,
                &addr_str,
                &asm,
            ]);
        }
        out.push_str(&format!("{}\n", output_table));

        out
    }
}

fn format_bool(val: bool) -> String {
    if val {
        "yes".green().to_string()
    } else {
        "no".dimmed().to_string()
    }
}

fn truncate_str(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max])
    }
}
