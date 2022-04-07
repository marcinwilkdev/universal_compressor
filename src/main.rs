use universal_compressor::{EliasOmegaDecoder, EliasOmegaEncoder};

fn main() {
    let data = std::fs::read("txt_file").expect("file doesnt exist");

    let encoded = universal_compressor::encode::<EliasOmegaEncoder>(&data);

    let decoded = universal_compressor::decode::<EliasOmegaDecoder>(&encoded);

    let text = String::from_utf8_lossy(&decoded);

    println!("{}", text);
}
