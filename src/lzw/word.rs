pub struct Word {
    symbols: Vec<u8>,
}

impl Word {
    pub fn new() -> Word {
        Word {
            symbols: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: u8) {
        self.symbols.push(symbol);
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    pub fn get_last_symbol(&self) -> u8 {
        *self.symbols.iter().last().expect("the word is empty")
    }

    pub fn get_symbols(self) -> Vec<u8> {
        self.symbols
    }

    pub fn get_symbols_ref(&self) -> &[u8] {
        &self.symbols
    }

    pub fn without_last_symbol(&self) -> Word {
        Word {
            symbols: self.symbols[..self.symbols.len() - 1].to_vec(),
        }
    }
}

