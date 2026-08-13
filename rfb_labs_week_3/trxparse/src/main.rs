use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

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


//  outputs: [
//         TxOut1 {
//             value: 100,
//             script_pubkey: ...
//         },
//         TxOut2 {
//             value: 100,
//             script_pubkey: ...
//         },
//  ]

// read_u64: read the next 8 bytes
// read_u32: read the next 4 bytes
// read_u16: read the next 2 bytes
// read_u8: read the next 1 byte 


// This function is a fundamental building block in a Bitcoin parser because CompactSize integers are used throughout the protocol to 
// encode the number of transaction inputs, outputs, script lengths, witness element counts, and many other variable-length fields.

fn read_varint(r: &mut Cursor<Vec<u8>>) -> u64 {
    let n = r.read_u8().unwrap();
    match n {
        0x00..=0xfc => n as u64,
        0xfd => r.read_u16::<LittleEndian>().unwrap() as u64,
        0xfe => r.read_u32::<LittleEndian>().unwrap() as u64,
        _ => r.read_u64::<LittleEndian>().unwrap(),
    }
}



fn read_bytes(r: &mut Cursor<Vec<u8>>, n: usize) -> Vec<u8> {
    let mut b = vec![0; n]; // creates a Vec<u8> with 32 elements, where each element is one byte (u8).
    r.read_exact(&mut b).unwrap(); // read exactly 32 bytes from the last position of the cursor, 
    // The cursor moves forward by 32 bytes.
    b
}

fn hex(v: &[u8]) -> String {
    v.iter().map(|b| format!("{:02x}", b)).collect()
}

fn main() {
    let raw = "0200000000010196277c04c986c1ad78c909287fd12dba2924324699a0232e0533f46a6a3916bb0100000000ffffffff026400000000000000160014274ae586ad2035efb4c25049c155f98310d7e106ca16440000000000160014599bcef6387256c6b019030c421b4a4d382fe2600247304402204d94a1e4047ca38a450177ccb6f88585ca147f1939df343d8ac5d962c5f35bb302206f7fa42c21c47ebccdc460393d35c5dfd3b6f0a26cf10fac23d3e6fab71835c20121020cb972a66e3fb1cdcc9efcad060b4457ebec534942700d4af1c0d82a33aa13f100000000";
    // version: 02000000
  
   let bytes = hex::decode(raw).unwrap();

     let mut r = Cursor::new(bytes);

       // When we read 4 bytes, the cursor automatically moves forward:
    // read_u32 : Read the next 4 bytes and interpret them as an unsigned 32-bit integer.
    // LittleEndian:  With little-endian, the least significant byte comes first.
    // Bitcoin  transaction version is a 4-byte little-endian integer.
    // we have 02 00 00 00 little-endian  turn it into 00 00 00 02: 0x00000002

     let version = r.read_u32::<LittleEndian>().unwrap();
     println!("Version: {}", version);

    // read_u8() consumes exactly one byte : 00
    // u8 means an unsigned 8-bit integer : 8 bits = 1 byte

    let marker = r.read_u8().unwrap();
    let flag = r.read_u8().unwrap();
      // marker and flag are each one byte.
    // marker and flag tell us that this is SegWit transaction, and witness data is present.

    println!("SegWit marker={} flag={}", marker, flag);

    let in_count = read_varint(&mut r);

     println!("Inputs: {}", in_count);

    for i in 0..in_count {
        println!("Input {}", i);
         // 32-byte previous transaction ID from the input. (from the current position of the cursor)
        // From the current cursor position, read exactly 32 bytes
        // Bitcoin transaction input contains a 32-byte previous transaction hash (TXID).
        let prev = read_bytes(&mut r, 32);
          println!("  Prev TXID (LE): {}", hex(&prev));
        let vout = r.read_u32::<LittleEndian>().unwrap(); 
         println!("  Vout: {}", vout);
       let slen = read_varint(&mut r) as usize;
        println!("  script length: {}", slen);
        
        let script = read_bytes(&mut r,slen);
        println!("  ScriptSig: {}", hex(&script)); 
        let seq = r.read_u32::<LittleEndian>().unwrap();
        println!("  Sequence: {:08x}", seq);
    }

    let out_count = read_varint(&mut r);

    println!("Outputs: {}", out_count);
    for i in 0..out_count {
        println!("Output {}", i);
        let value = r.read_u64::<LittleEndian>().unwrap(); // Read the next 8 bytes and interpret them as a little-endian u64.
        println!("  Value: {} sats", value);
        let slen = read_varint(&mut r) as usize;
        println!("  script length: {}", slen);
        let script = read_bytes(&mut r,slen);
        println!("  ScriptPubKey: {}", hex(&script));
    }

       // each input has its own witness field,
    for i in 0..in_count {
        let items = read_varint(&mut r);
        println!("Witness for input {} ({} item(s))", i, items);
        for j in 0..items {
            let len = read_varint(&mut r) as usize;
            let item = read_bytes(&mut r,len);
            println!("  Item {}: {}", j, hex(&item));
        }
    }

    let locktime = r.read_u32::<LittleEndian>().unwrap();
    println!("Locktime: {}", locktime);


}