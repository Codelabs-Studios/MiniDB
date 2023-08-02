use std::io::{Read, Write};

use zstd::stream::{read::Decoder, write::Encoder};

pub fn encode(data: &str) -> Vec<u8> {
    let mut encoder = Encoder::new(Vec::new(), 0).unwrap();
    encoder.write_all(data.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

pub fn decode(data: Vec<u8>) -> String {
    let mut decoder = Decoder::new(data.as_slice()).unwrap();
    let mut decoded = String::new();
    decoder.read_to_string(&mut decoded).unwrap();
    decoded
}
