pub mod bits;
pub mod lzw;
pub mod number_encoders;

pub use number_encoders::{NumberDecoder, NumberEncoder};

use bits::Bits;
use lzw::lzw_decoder::LzwDecoder;
use lzw::lzw_encoder::LzwEncoder;

pub use number_encoders::elias::omega::{EliasOmegaDecoder, EliasOmegaEncoder};
pub use number_encoders::elias::delta::{EliasDeltaDecoder, EliasDeltaEncoder};
pub use number_encoders::elias::gamma::{EliasGammaDecoder, EliasGammaEncoder};
pub use number_encoders::fibbonaci::{FibbonaciDecoder, FibbonaciEncoder};

pub fn encode<E>(data: &[u8]) -> Bits where E: NumberEncoder {
    let encoded_lzw = LzwEncoder::new().encode_text(&data);

    // Needed because elias can't handle 0.
    let encoded_lzw: Vec<_> = encoded_lzw.into_iter().map(|s| s + 1).collect();

    let encoded_numbers = E::encode(&encoded_lzw);

    encoded_numbers
}

pub fn decode<D>(data: &Bits) -> Vec<u8> where D: NumberDecoder {
    let decoded_numbers = D::decode(&data);

    // Needed because elias can't handle 0.
    let decoded_numbers: Vec<_> = decoded_numbers.into_iter().map(|s| s - 1).collect();

    let decoded = LzwDecoder::new().decode_text(&decoded_numbers);

    decoded
}
