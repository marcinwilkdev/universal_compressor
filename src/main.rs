// use universal_compressor::{EliasOmegaDecoder, EliasOmegaEncoder};
// use universal_compressor::{EliasDeltaEncoder, EliasDeltaDecoder};
use universal_compressor::{EliasGammaEncoder, EliasGammaDecoder};

fn main() {
    let data = std::fs::read("txt_file").expect("file doesnt exist");

    let encoded = universal_compressor::encode::<EliasGammaEncoder>(&data);

    let decoded = universal_compressor::decode::<EliasGammaDecoder>(&encoded);

    let text = String::from_utf8_lossy(&decoded);

    println!("{}", text);
}
