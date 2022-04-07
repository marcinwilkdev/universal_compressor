//! Things usefull for decoding LZW encoded data.

use crate::lzw::{self, Dictionary, Word};

/// Used to decode LZW encoded data.
pub struct LzwDecoder {
    dictionary: Dictionary,
    last_word: Option<Word>,
}

impl LzwDecoder {
    /// Creates new instance of `LzwDecoder` with dictionary initialized
    /// to all ASCII symbols.
    pub fn new() -> LzwDecoder {
        LzwDecoder {
            dictionary: lzw::create_dictionary(),
            last_word: None,
        }
    }

    /// Decodes LZW encoded `codes` into `Vec<u8>`.
    pub fn decode_text(&mut self, text: &[usize]) -> Vec<u8> {
        let mut codes = text.into_iter().map(|n| *n);

        let mut words = Vec::new();

        while let Some(word) = self.get_next_word(&mut codes) {
            words.push(word);
        }

        words
            .into_iter()
            .map(Word::get_symbols)
            .flatten()
            .collect()
    }

    /// Fetches next code from `codes` iterator, transforms it into
    /// word and updates dictionary to handle the rest of codes.
    fn get_next_word<I>(&mut self, codes: &mut I) -> Option<Word>
    where
        I: Iterator<Item = usize>,
    {
        let code = codes.next()?;

        match self.find_word(code) {
            Some(word) => self.word_in_dictionary(word),
            None => self.word_not_in_dictionary(),
        }
    }

    /// If last word exists it gets updated with first symbol
    /// of current word. Sets word as new last word.
    fn word_in_dictionary(&mut self, word: Word) -> Option<Word> {
        if let Some(mut last_word) = self.last_word.take() {
            last_word.add_symbol(word.get_first_symbol());
            self.dictionary.push(last_word);
        }

        self.last_word = Some(word.clone());

        Some(word)
    }

    /// Updates last word with first symbol of itself, adds last word
    /// to dictionary and sets updated last word as new last word.
    fn word_not_in_dictionary(&mut self) -> Option<Word> {
        let mut last_word = self.last_word.take().expect("there has to exist last word");
        last_word.add_symbol(last_word.get_first_symbol());

        self.dictionary.push(last_word.clone());
        self.last_word = Some(last_word.clone());

        Some(last_word)
    }

    /// Finds word in dictionary.
    fn find_word(&self, code: usize) -> Option<Word> {
        self.dictionary.get(code).map(|w| w.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lzw_decode_works() {
        let mut lzw_decode_dict = LzwDecoder::new();
        let codes = [0, 1, 256, 258, 257, 1];

        let words = lzw_decode_dict.decode_text(&codes);

        assert_eq!(vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1], words);
    }
}
