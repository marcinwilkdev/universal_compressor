pub struct LzwDictionary {
    // words encoded as vec of bytes
    dict: Vec<Vec<u8>>,
    last_symbol: Option<u8>,
}

impl LzwDictionary {
    pub fn new() -> Self {
        let dict = (0..128).map(|n| vec![n]).collect();

        LzwDictionary {
            dict,
            last_symbol: None,
        }
    }

    pub fn get_next_code<I>(&mut self, symbols: &mut I) -> Option<usize>
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
                let (index, _) = self.find_word(&word).expect("has to be some");

                if word.len() == 1 {
                    self.last_symbol = None;
                } else {
                    self.last_symbol = Some(*word.iter().last().expect("has to be some"));
                }

                return Some(index);
            }
        }

        self.last_symbol = Some(*word.iter().last().expect("has to be some"));

        let (index, _) = self
            .find_word(&word[..word.len() - 1])
            .expect("has to be some");

        self.push_word(word);

        Some(index)
    }

    pub fn get_dict(&self) -> &[Vec<u8>] {
        &self.dict
    }

    fn push_word(&mut self, word: Vec<u8>) {
        self.dict.push(word);
    }

    fn find_word(&self, word: &[u8]) -> Option<(usize, &Vec<u8>)> {
        self.dict.iter().enumerate().find(|&(_, w)| *w == word)
    }
}

pub struct LzwDecodeDictionary {
    dict: Vec<Vec<u8>>,
    last_word: Option<Vec<u8>>,
}

impl LzwDecodeDictionary {
    pub fn new() -> LzwDecodeDictionary {
        let dict = (0..128).map(|n| vec![n]).collect();

        LzwDecodeDictionary {
            dict,
            last_word: None,
        }
    }

    pub fn get_next_word<I>(&mut self, codes: &mut I) -> Option<Vec<u8>>
    where
        I: Iterator<Item = usize>,
    {
        let code = match codes.next() {
            Some(c) => c,
            None => return None,
        };

        match self.find_word(code) {
            Some(w) => {
                if let Some(mut last_word) = self.last_word.take() {
                    last_word.push(w[0]);
                    self.dict.push(last_word);
                }

                self.last_word = Some(w.clone());

                Some(w)
            }
            None => {
                let mut last_word = self.last_word.take().expect("has to be some");
                last_word.push(last_word[0]);

                self.last_word = Some(last_word.clone());

                Some(last_word)
            }
        }
    }

    fn find_word(&self, code: usize) -> Option<Vec<u8>> {
        self.dict.get(code).map(|v| v.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization_works() {
        let lzw_dict = LzwDictionary::new();

        assert_eq!(128, lzw_dict.get_dict().len());
    }

    #[test]
    fn next_code_works() {
        let mut lzw_dict = LzwDictionary::new();
        let symbols = [0, 1, 2];
        let mut symbols_iter = symbols.into_iter();

        let code = lzw_dict.get_next_code(&mut symbols_iter);
        let code_two = lzw_dict.get_next_code(&mut symbols_iter);

        assert_eq!(Some(0), code);
        assert_eq!(Some(1), code_two);
    }

    #[test]
    fn lzw_works() {
        let mut lzw_dict = LzwDictionary::new();
        let symbols = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let mut symbols_iter = symbols.into_iter();

        let mut codes = vec![];

        while let Some(c) = lzw_dict.get_next_code(&mut symbols_iter) {
            codes.push(c);
        }

        assert_eq!(vec![0, 1, 128, 130, 129, 1], codes);
    }

    #[test]
    fn lzw_decode_works() {
        let mut lzw_decode_dict = LzwDecodeDictionary::new();
        let codes = [0, 1, 128, 130, 129, 1];
        let mut codes_iter = codes.into_iter();

        let mut words = vec![];

        while let Some(w) = lzw_decode_dict.get_next_word(&mut codes_iter) {
            words.push(w);
        }

        assert_eq!(
            vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            words.into_iter().flatten().collect::<Vec<_>>()
        );
    }
}
