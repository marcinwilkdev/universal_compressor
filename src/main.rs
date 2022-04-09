// use universal_compressor::{EliasOmegaDecoder, EliasOmegaEncoder};
// use universal_compressor::{EliasDeltaEncoder, EliasDeltaDecoder};
// use universal_compressor::{EliasGammaEncoder, EliasGammaDecoder};
use universal_compressor::{FibbonaciEncoder, FibbonaciDecoder};


fn main() {
    let data = std::fs::read("txt_file").expect("file doesnt exist");

    let encoded = universal_compressor::encode::<FibbonaciEncoder>(&data);

    let decoded = universal_compressor::decode::<FibbonaciDecoder>(&encoded);

    let text = String::from_utf8_lossy(&decoded);

    println!("{}", text);
}
