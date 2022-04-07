// use universal_compressor::{EliasOmegaDecoder, EliasOmegaEncoder};
use universal_compressor::{EliasDeltaEncoder, EliasDeltaDecoder};

fn main() {
    let data = std::fs::read("txt_file").expect("file doesnt exist");

    let encoded = universal_compressor::encode::<EliasDeltaEncoder>(&data);

    let decoded = universal_compressor::decode::<EliasDeltaDecoder>(&encoded);

    let text = String::from_utf8_lossy(&decoded);

    println!("{}", text);
}
