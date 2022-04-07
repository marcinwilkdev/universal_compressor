pub mod lzw_decoder;
pub mod lzw_encoder;
pub mod word;

use std::collections::BTreeMap;

pub const ALPHABET_SIZE: u8 = 255;

/// Creates initial dictionary for LzwDecoder.
pub fn create_dictionary() -> Vec<Vec<u8>> {
    (0..=ALPHABET_SIZE).map(|n| vec![n]).collect()
}

/// Creates initial dictionary for LzwEncoder.
pub fn create_btree_dictionary() -> BTreeMap<Vec<u8>, usize> {
    (0..=ALPHABET_SIZE).map(|n| (vec![n], n as usize)).collect()
}
