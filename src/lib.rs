pub mod bits;
pub mod elias_omega;
pub mod lzw;

use bits::Bits;
use lzw::lzw_decoder::LzwDecoder;
use lzw::lzw_encoder::LzwEncoder;

pub fn encode(data: &[u8]) -> Bits {
    let encoded = LzwEncoder::new().encode_text(&data);

    let encoded: Vec<_> = encoded.into_iter().map(|s| s + 1).collect();

    let mut bits = Bits::new();

    for e in &encoded {
        elias_omega::encode_number_and_append(*e, &mut bits);
    }

    bits
}

pub fn decode(data: &Bits) -> Vec<u8> {
    let decoded = elias_omega::decode_number(&data);

    let decoded: Vec<_> = decoded.into_iter().map(|s| s - 1).collect();

    let mut decoded_iter = decoded.into_iter();

    let decoded = LzwDecoder::new().decode_text(&mut decoded_iter);

    decoded
}
