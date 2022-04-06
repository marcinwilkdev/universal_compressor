//! Things usefull for encoding LZW encoded data.

use crate::lzw;

/// Used to encode LZW encoded data.
pub struct LzwEncoder {
    // words encoded as vec of bytes
    dictionary: Vec<Vec<u8>>,
    last_symbol: Option<u8>,
}

impl LzwEncoder {
    /// Creates new instance of `LzwEncoder` with dictionary initialized
    /// to all ASCII symbols.
    pub fn new() -> Self {
        let dictionary = lzw::create_dictionary();

        LzwEncoder {
            dictionary,
            last_symbol: None,
        }
    }

    /// Encodes `symbols` using LZW encoding into `Vec<usize>`.
    pub fn encode_text<I>(&mut self, symbols: &mut I) -> Vec<usize>
    where
        I: Iterator<Item = u8>,
    {
        let mut codes = vec![];

        while let Some(c) = self.get_next_code(symbols) {
            codes.push(c);
        }

        codes
    }

    /// Gets next code from `symbols` iterator and updates dictionary.
    fn get_next_code<I>(&mut self, symbols: &mut I) -> Option<usize>
    where
        I: Iterator<Item = u8>,
    {
        let mut word = vec![];

        if let Some(last_symbol) = self.last_symbol {
            word.push(last_symbol);
        } else {
            let symbol = symbols.next();

            if let Some(symbol) = symbol {
                word.push(symbol);
            } else {
                return None;
            }
        }

        while self.find_word(&word).is_some() {
            let symbol = symbols.next();

            if let Some(symbol) = symbol {
                word.push(symbol);
            } else {
                let index = self.get_word_index(&word);

                if word.len() == 1 {
                    self.last_symbol = None;
                } else {
                    let last_symbol = LzwEncoder::get_last_word_symbol(&word);
                    self.last_symbol = Some(last_symbol);
                }

                return Some(index);
            }
        }

        let last_symbol = LzwEncoder::get_last_word_symbol(&word);
        let index = self.get_word_index(&word[..word.len() - 1]);

        self.last_symbol = Some(last_symbol);
        self.dictionary.push(word);

        Some(index)
    }

    /// Make sure that word is not empty !!!
    fn get_last_word_symbol(word: &[u8]) -> u8 {
        *word.iter().last().expect("has to be some")
    }

    // Make sure that word exists in dictionary !!!
    fn get_word_index(&self, word: &[u8]) -> usize {
        let (index, _) = self.find_word(word).expect("has to be some");

        index
    }

    // Searches for word in dictionary and returns it with its index.
    fn find_word(&self, word: &[u8]) -> Option<(usize, &Vec<u8>)> {
        self.dictionary
            .iter()
            .enumerate()
            .find(|&(_, w)| *w == word)
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
