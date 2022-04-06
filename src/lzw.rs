mod lzw_decoder;
mod lzw_encoder;

pub const ALPHABET_SIZE: u8 = 255;

/// Creates initial dictionary for LzwEncoder and
/// LzwDecoder.
pub fn create_dictionary() -> Vec<Vec<u8>> {
    (0..=255).map(|n| vec![n]).collect()
}
