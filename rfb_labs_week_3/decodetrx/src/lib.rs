use std::io::{Read, Error};
// use clap::{Parser, Subcommand};
use clap::{Arg, Command};
use std::fmt;
use sha2::{Sha256, Sha512, Digest}; // https://docs.rs/sha2/latest/sha2/
use transaction::{Amount, Input, Output, Transaction, Txid};
mod transaction;

// #[derive(Parser)]
// #[command(name= " Transaction decoder")]
// #[command(version= "1.0")]
// #[command(about= "Bitcoin Transaction decoder", long_about=None)]
// struct CLI {
//       #[arg(
//             required = true,
//             help="(string, required) Row Transaction hex"
//         )]
//     transaction_hex: String
// }


#[allow(unused_variables)]
fn read_version(transaction_hex: &str) -> u32 {
 
}


fn read_u64(transaction_bytes: &mut &[u8]) -> u64 {
  
}

fn read_amount(transaction_bytes: &mut &[u8]) -> Result<Amount, Error> {

}



fn read_u32(bytes_slice: &mut &[u8]) ->Result<u32, Error> {}
  


fn read_compact_size(transaction_bytes: &mut &[u8]) -> Result<u64, Error> {

 
}

fn read_txid(transaction_bytes: &mut &[u8]) -> Result<Txid, Error> {
  
}



fn read_script_size(transaction_bytes: &mut &[u8]) -> Result<String, Error> {

}

fn read_version_byte(transaction_bytes: &mut &[u8]) -> Result<u32, Error> {

}
// Bitcoin uses little-endian encoding for most of its numeric fields, meaning the least significant byte comes first.

fn hash_row_transaction(row_transaction_bytes: &[u8]) -> Result<Txid, Error> {


}


pub fn decode_transaction(transaction_hex: String) -> Result<String, Box<dyn std::error::Error>> {
    

}