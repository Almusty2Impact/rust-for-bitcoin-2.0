//! Bitcoin Transaction Explorer — CLI entry point.
//!
//! Decode, analyze and inspect any Bitcoin transaction field by field.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use tx_explorer::analysis;
use tx_explorer::decode::{build_decoded_tx, decode_hex};
use tx_explorer::output::ExplorationResult;
use tx_explorer::script::classify_outputs;

/// Bitcoin Transaction Explorer — decode and analyse any transaction.
#[derive(Parser, Debug)]
#[command(
    name = "txexp",
    version = "0.1.0",
    about = "Decode, analyse and inspect any Bitcoin transaction field by field",
    long_about = "A decoder and analyser for Bitcoin transactions. Given a txid (fetched from a node) \
                  or raw hex, it decodes every field, identifies script types, disassembles scripts \
                  into opcodes, and calculates size, weight, vsize, fee and fee rate."
)]
struct Cli {
    /// Output as JSON instead of human-readable tables.
    #[arg(long, short)]
    json: bool,

    /// Network for address derivation (regtest, signet, testnet4, bitcoin).
    #[arg(long, short, default_value = "regtest")]
    network: NetworkArg,

    #[command(subcommand)]
    command: Commands,
}

/// Supported network values.
#[derive(Debug, Clone, clap::ValueEnum)]
enum NetworkArg {
    Regtest,
    Signet,
    Testnet4,
    Bitcoin,
}

impl From<NetworkArg> for bitcoin::Network {
    fn from(val: NetworkArg) -> Self {
        match val {
            NetworkArg::Regtest => bitcoin::Network::Regtest,
            NetworkArg::Signet => bitcoin::Network::Signet,
            NetworkArg::Testnet4 => bitcoin::Network::Testnet4,
            NetworkArg::Bitcoin => bitcoin::Network::Bitcoin,
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Decode a raw transaction from hex string.
    #[command(alias = "decode")]
    Hex { raw_hex: String },

    /// Fetch and decode a transaction by txid from Bitcoin Core RPC.
    #[command(alias = "fetch")]
    Txid { txid: String },

    /// Fetch a txid via mempool.space — no Bitcoin Core node required.
    Api { txid: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let network: bitcoin::Network = cli.network.into();

    match cli.command {
        Commands::Hex { raw_hex } => {
            let tx = decode_hex(&raw_hex).context("Failed to decode raw transaction hex")?;
            let decoded = build_decoded_tx(&tx);
            let script_info = classify_outputs(
                &decoded
                    .outputs
                    .iter()
                    .map(|o| o.script_pubkey_hex.clone())
                    .collect::<Vec<_>>(),
                network,
            );
            let analysis_result = analysis::analyze(&decoded);

            let result = ExplorationResult {
                decoded,
                script_info,
                fee: None, // Fee requires RPC for previous outputs
                analysis: analysis_result,
                confirmations: None,
            };

            if cli.json {
                println!("{}", result.to_json()?);
            } else {
                println!("{}", result.to_terminal());
            }
        }
        Commands::Txid { txid } => {
            // Load .env for RPC credentials
            let _ = dotenvy::dotenv();

            let rpc_url =
                std::env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:18443".to_string());
            let rpc_user = std::env::var("RPC_USER").unwrap_or_else(|_| "user".to_string());
            let rpc_pass = std::env::var("RPC_PASS").unwrap_or_else(|_| "pass".to_string());

            let rpc = tx_explorer::rpc::BitcoinRpc::connect(&rpc_url, &rpc_user, &rpc_pass)
                .context("Failed to connect to Bitcoin Core RPC")?;

            let txid_parsed: bitcoin::Txid = txid.parse().context("Invalid txid format")?;

            let tx = rpc
                .get_transaction(&txid_parsed)
                .context("Transaction not found on node")?;

            let decoded = build_decoded_tx(&tx);
            let script_info = classify_outputs(
                &decoded
                    .outputs
                    .iter()
                    .map(|o| o.script_pubkey_hex.clone())
                    .collect::<Vec<_>>(),
                network,
            );
            let analysis_result = analysis::analyze(&decoded);

            // Calculate fee by fetching previous outputs
            let fee = rpc.calculate_fee_from_prev_outputs(&tx, decoded.vsize).ok();

            // Get confirmation count
            let confirmations = rpc.get_confirmations(&txid_parsed).ok();

            let result = ExplorationResult {
                decoded,
                script_info,
                fee,
                analysis: analysis_result,
                confirmations,
            };

            if cli.json {
                println!("{}", result.to_json()?);
            } else {
                println!("{}", result.to_terminal());
            }
        }
        Commands::Api { txid } => {
            let mem = tx_explorer::mempool::MempoolSpace::new();

            let raw_hex = mem
                .raw_hex(&txid)
                .context("Failed to fetch hex from mempool.space")?;
            let tx = decode_hex(&raw_hex).context("Failed to decode fetched transaction")?;
            let decoded = build_decoded_tx(&tx);

            let script_info = classify_outputs(
                &decoded
                    .outputs
                    .iter()
                    .map(|o| o.script_pubkey_hex.clone())
                    .collect::<Vec<_>>(),
                network,
            );
            let analysis_result = analysis::analyze(&decoded);

            let fee = if decoded.is_coinbase {
                None
            } else {
                mem.fetch_fee(&txid).ok()
            };
            let confirmations = mem.confirmations(&txid).ok();

            let result = ExplorationResult {
                decoded,
                script_info,
                fee,
                analysis: analysis_result,
                confirmations,
            };

            if cli.json {
                println!("{}", result.to_json()?);
            } else {
                println!("{}", result.to_terminal());
            }
        }

    }

    Ok(())
}
