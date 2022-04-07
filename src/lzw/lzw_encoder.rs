//! Things usefull for encoding LZW encoded data.

use crate::lzw::word::Word;
use crate::lzw::{self, HashMapDictionary, ALPHABET_SIZE};

/// Used to encode LZW encoded data.
pub struct LzwEncoder {
    dictionary: HashMapDictionary,
    word_code: usize,
    last_symbol: Option<u8>,
}

impl LzwEncoder {
    /// Creates new instance of `LzwEncoder` with dictionary initialized
    /// to all ASCII symbols.
    pub fn new() -> Self {
        LzwEncoder {
            dictionary: lzw::create_hashmap_dictionary(),
            word_code: ALPHABET_SIZE as usize + 1,
            last_symbol: None,
        }
    }

    /// Encodes `symbols` using LZW encoding into `Vec<usize>`.
    pub fn encode_text(&mut self, text: &[u8]) -> Vec<usize> {
        let mut symbols = text.iter().map(|s| *s);

        let mut codes = vec![];

        while let Some(code) = self.get_next_code(&mut symbols) {
            codes.push(code);
        }

        codes
    }

    /// Gets next code from `symbols` iterator and updates dictionary.
    fn get_next_code<I>(&mut self, symbols: &mut I) -> Option<usize>
    where
        I: Iterator<Item = u8>,
    {
        let mut curr_word = Word::new();

        match self.last_symbol {
            Some(last_symbol) => curr_word.add_symbol(last_symbol),
            None => curr_word.add_symbol(symbols.next()?),
        }

        while self.find_word(&curr_word).is_some() {
            match symbols.next() {
                Some(symbol) => curr_word.add_symbol(symbol),
                None => return self.end_encoding(curr_word),
            }
        }

        let code = self.get_word_code(&curr_word.without_last_symbol());

        self.last_symbol = Some(curr_word.get_last_symbol());
        self.dictionary.insert(curr_word, self.word_code);
        self.word_code += 1;

        Some(code)
    }

    // resets last symbol to end loop and returns last word code
    fn end_encoding(&mut self, curr_word: Word) -> Option<usize> {
        self.last_symbol = None;

        Some(self.get_word_code(&curr_word))
    }

    // Make sure that word exists in dictionary !!!
    fn get_word_code(&self, word: &Word) -> usize {
        let (_, code) = self.find_word(word).expect("word not in dictionary");

        code
    }

    // Finds word in the dictionary.
    fn find_word(&self, word: &Word) -> Option<(&Word, usize)> {
        self.dictionary.get_key_value(word).map(|(a, b)| (a, *b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization_works() {
        let lzw_dict = LzwEncoder::new();

        assert_eq!(256, lzw_dict.dictionary.len());
    }

    #[test]
    fn next_code_works() {
        let mut lzw_dict = LzwEncoder::new();
        let symbols = [0, 1, 2];
        let mut symbols_iter = symbols.into_iter();

        let code = lzw_dict.get_next_code(&mut symbols_iter);
        let code_two = lzw_dict.get_next_code(&mut symbols_iter);

        assert_eq!(Some(0), code);
        assert_eq!(Some(1), code_two);
    }

    #[test]
    fn lzw_works() {
        let mut lzw_dict = LzwEncoder::new();
        let symbols = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1];

        let codes = lzw_dict.encode_text(&symbols);

        assert_eq!(vec![0, 1, 256, 258, 257, 1], codes);
    }
}
