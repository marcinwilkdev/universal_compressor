//! `Word` structure usefull for LZW encoding and decoding.

/// Represents words in ASCII format as `Vec<u8>`.
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Word {
    symbols: Vec<u8>,
}

impl Word {
    /// Creates new empty instance of `Word`.
    pub fn new() -> Word {
        Word {
            symbols: Vec::new(),
        }
    }

    /// Adds one ASCII symbol to the end of the `Word`.
    pub fn add_symbol(&mut self, symbol: u8) {
        self.symbols.push(symbol);
    }

    /// Returns number of ASCII symbols in `Word`.
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Returns last ASCII symbol from the `Word`.
    pub fn get_last_symbol(&self) -> u8 {
        *self.symbols.iter().last().expect("the word is empty")
    }
    
    /// Return first ASCII symbol from the `Word`.
    pub fn get_first_symbol(&self) -> u8 {
        *self.symbols.get(0).expect("the word is empty")
    }

    /// Returns `Word`'s ASCII symbols as `Vec<u8>`.
    /// Consumes `Word` so it can't be used later.
    pub fn get_symbols(self) -> Vec<u8> {
        self.symbols
    }

    /// Returns `Word`'s ASCII symbols as `&[u8]`.
    pub fn get_symbols_ref(&self) -> &[u8] {
        &self.symbols
    }

    /// Returns new `Word` which is this `Word` without last symbol.
    pub fn without_last_symbol(&self) -> Word {
        Word {
            symbols: self.symbols[..self.symbols.len() - 1].to_vec(),
        }
    }

    pub fn from_vec(symbols: Vec<u8>) -> Word {
        Word {
            symbols,
        }
    }
}
