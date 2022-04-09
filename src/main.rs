use std::path::PathBuf;
use structopt::StructOpt;

use universal_compressor::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "universal_compressor")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
    #[structopt(short, long)]
    decode: bool,
    #[structopt(short, long)]
    encoding: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let data = std::fs::read(opt.file).expect("file doesnt exist");

    if opt.decode {
        let (size_bytes, rest) = data.split_at(std::mem::size_of::<usize>());
        let size = usize::from_be_bytes(size_bytes.try_into().unwrap());
        let bytes = rest.to_vec();

        let bits = Bits::from_vec(size, bytes);

        assert_eq!("fib", "fib".to_string());

        let decoded = match opt.encoding {
            Some(e) if e == "fib" => universal_compressor::decode::<FibbonaciDecoder>(&bits),
            Some(e) if e == "gamma" => universal_compressor::decode::<EliasGammaDecoder>(&bits),
            Some(e) if e == "delta" => universal_compressor::decode::<EliasDeltaDecoder>(&bits),
            _ => universal_compressor::decode::<EliasOmegaDecoder>(&bits),
        };

        std::fs::write(opt.output, &decoded).expect("couldn't write output");
    } else {
        let encoded = match opt.encoding {
            Some(e) if e == "fib" => universal_compressor::encode::<FibbonaciEncoder>(&data),
            Some(e) if e == "gamma" => universal_compressor::encode::<EliasGammaEncoder>(&data),
            Some(e) if e == "delta" => universal_compressor::encode::<EliasDeltaEncoder>(&data),
            _ => universal_compressor::encode::<EliasOmegaEncoder>(&data),
        };

        let mut bytes = encoded.len().to_be_bytes().to_vec();

        bytes.append(&mut encoded.get_bits().to_vec());

        std::fs::write(opt.output, bytes).expect("couldn't write output");

        let data_len = data.len();
        let encoded_len = encoded.len() / 8;

        let compression_ratio = encoded_len as f64 / data_len as f64;

        println!("Encoded file len (bytes): {}", data_len);
        println!("Encoded code len (bytes): {}", encoded_len);
        println!("Compression ratio: {}", compression_ratio);
        println!("Encoded file entropy: {}", 0);
        println!("Encoded code entropy: {}", 0);
    }
}
