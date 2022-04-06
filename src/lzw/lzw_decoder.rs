//! Things usefull for decoding LZW encoded data.

use crate::lzw;

/// Used to decode LZW encoded data.
pub struct LzwDecoder {
    dictionary: Vec<Vec<u8>>,
    last_word: Option<Vec<u8>>,
}

impl LzwDecoder {
    /// Creates new instance of `LzwDecoder` with dictionary initialized
    /// to all ASCII symbols.
    pub fn new() -> LzwDecoder {
        let dict = lzw::create_dictionary();

        LzwDecoder {
            dictionary: dict,
            last_word: None,
        }
    }

    /// Decodes LZW encoded `codes` into `Vec<u8>`.
    pub fn decode_text<I>(&mut self, codes: &mut I) -> Vec<u8>
    where
        I: Iterator<Item = usize>,
    {
        let mut words = Vec::new();

        while let Some(w) = self.get_next_word(codes) {
            words.push(w);
        }

        words.into_iter().flatten().collect()
    }

    /// Fetches next code from `codes` iterator, transforms it into
    /// word and updates dictionary to handle the rest of codes.
    fn get_next_word<I>(&mut self, codes: &mut I) -> Option<Vec<u8>>
    where
        I: Iterator<Item = usize>,
    {
        let code = match codes.next() {
            Some(c) => c,
            None => return None,
        };

        match self.find_word(code) {
            Some(w) => self.word_in_dictionary(w),
            None => self.word_not_in_dictionary(),
        }
    }

    /// If there exists last word updates it with first symbol
    /// of current word. Sets word as new last word.
    fn word_in_dictionary(&mut self, word: Vec<u8>) -> Option<Vec<u8>> {
        if let Some(mut last_word) = self.last_word.take() {
            last_word.push(word[0]);
            self.dictionary.push(last_word);
        }

        self.last_word = Some(word.clone());

        Some(word)
    }

    /// Updates last word with first symbol of itself, adds last word
    /// to dictionary and sets updated last word as new last word.
    fn word_not_in_dictionary(&mut self) -> Option<Vec<u8>> {
        let mut last_word = self.last_word.take().expect("has to be some");
        last_word.push(last_word[0]);

        self.dictionary.push(last_word.clone());
        self.last_word = Some(last_word.clone());

        Some(last_word)
    }

    /// Searches for word with `code` in dictionary.
    fn find_word(&self, code: usize) -> Option<Vec<u8>> {
        self.dictionary.get(code).map(|v| v.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lzw_decode_works() {
        let mut lzw_decode_dict = LzwDecoder::new();
        let codes = [0, 1, 256, 258, 257, 1];
        let mut codes_iter = codes.into_iter();

        let words = lzw_decode_dict.decode_text(&mut codes_iter);

        assert_eq!(vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1], words);
    }
}
