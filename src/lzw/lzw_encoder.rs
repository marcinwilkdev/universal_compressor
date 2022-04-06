//! Things usefull for encoding LZW encoded data.

use std::collections::BTreeMap;

use crate::lzw;
use crate::lzw::word::Word;

/// Used to encode LZW encoded data.
pub struct LzwEncoder {
    dictionary: BTreeMap<Vec<u8>, usize>,
    word_code: usize,
    last_symbol: Option<u8>,
}

impl LzwEncoder {
    /// Creates new instance of `LzwEncoder` with dictionary initialized
    /// to all ASCII symbols.
    pub fn new() -> Self {
        let dictionary = lzw::create_btree_dictionary();

        LzwEncoder {
            dictionary,
            word_code: 256,
            last_symbol: None,
        }
    }

    /// Encodes `symbols` using LZW encoding into `Vec<usize>`.
    pub fn encode_text(&mut self, symbols: &[u8]) -> Vec<usize>
    {
        let mut symbols_iterator = symbols.iter().map(|s| *s);

        let mut codes = vec![];

        while let Some(code) = self.get_next_code(&mut symbols_iterator) {
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
                None => {
                    self.last_symbol = if curr_word.len() == 1 {
                        None
                    } else {
                        Some(curr_word.get_last_symbol())
                    };

                    let code = self.get_word_code(&curr_word);

                    return Some(code);
                }
            }
        }

        let code = self.get_word_code(&curr_word.without_last_symbol());

        self.last_symbol = Some(curr_word.get_last_symbol());
        self.dictionary
            .insert(curr_word.get_symbols(), self.word_code);
        self.word_code += 1;

        Some(code)
    }

    // Make sure that word exists in dictionary !!!
    fn get_word_code(&self, word: &Word) -> usize {
        let (_, code) = self.find_word(word).expect("word not in dictionary");

        code
    }

    // Searches for word in dictionary and returns it with its index.
    fn find_word(&self, word: &Word) -> Option<(&Vec<u8>, usize)> {
        self.dictionary
            .get_key_value(word.get_symbols_ref())
            .map(|(a, b)| (a, *b))
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
        let mut symbols_iter = symbols.into_iter();

        let codes = lzw_dict.encode_text(&mut symbols_iter);

        assert_eq!(vec![0, 1, 256, 258, 257, 1], codes);
    }
}
