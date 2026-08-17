use clap::{Arg, Command};
use decodetrx::decode_transaction;

fn main() {
    let matches = Command::new("Transaction Decoder")
        .version("1.0")
        .about("Decodes a raw Bitcoin transaction hex into JSON")
        .arg(
            Arg::new("transaction_hex")
                .required(true)
                .help("Raw transaction hex string"),
        )
        .get_matches();

    let hex = matches
        .get_one::<String>("transaction_hex")
        .expect("required arg");

    match decode_transaction(hex.to_string()) {
        Ok(json) => println!("{json}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

// https://mempool.space/testnet/tx/3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2
