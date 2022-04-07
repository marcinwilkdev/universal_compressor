pub mod lzw_decoder;
pub mod lzw_encoder;
pub mod word;

use std::collections::HashMap;

use word::Word;

pub const ALPHABET_SIZE: u8 = 255;

pub type Dictionary = Vec<Word>;
pub type HashMapDictionary = HashMap<Word, usize>;

/// Creates initial dictionary for LzwDecoder.
pub fn create_dictionary() -> Dictionary {
    (0..=ALPHABET_SIZE).map(|n| Word::from_vec(vec![n])).collect()
}

/// Creates initial dictionary for LzwEncoder.
pub fn create_hashmap_dictionary() -> HashMapDictionary {
    (0..=ALPHABET_SIZE).map(|n| (Word::from_vec(vec![n]), n as usize)).collect()
}
