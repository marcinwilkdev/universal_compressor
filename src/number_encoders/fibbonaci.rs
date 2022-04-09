//! Things for implementing fibbonaci encoding.

use crate::bits::{Bit, Bits};
use crate::number_encoders::{NumberDecoder, NumberEncoder};

/// Struct holding cached fibbonaci numbers used for fibbonaci encoding.
pub struct Fibbonaci {
    cache: Vec<usize>,
}

impl Fibbonaci {
    pub fn new() -> Fibbonaci {
        Fibbonaci { cache: vec![1, 2] }
    }

    pub fn find_greater_index(&mut self, number: usize) -> usize {
        // index and value
        let mut curr_index = 1;

        while self.get(curr_index) <= number {
            curr_index += 1;
        }

        curr_index
    }

    fn get(&mut self, index: usize) -> usize {
        if index >= self.cache.len() {
            self.adjust_cache(index);
        }

        self.cache[index]
    }

    fn adjust_cache(&mut self, index: usize) {
        let curr_len = self.cache.len();

        for i in curr_len..=index {
            let next_number = self.cache[i - 2] + self.cache[i - 1];
            self.cache.push(next_number);
        }
    }
}

/// Fibbonaci encoder.
pub struct FibbonaciEncoder;

impl NumberEncoder for FibbonaciEncoder {
    fn encode(numbers: &[usize]) -> Bits {
        let mut bits = Bits::new();
        let mut fibbonaci = Fibbonaci::new();

        for &(mut number) in numbers {
            let mut curr_bits = Bits::new();

            let curr_bits_len = fibbonaci.find_greater_index(number);

            for _ in 0..curr_bits_len {
                curr_bits.push_bit(Bit::ZERO);
            }

            curr_bits.push_bit(Bit::ONE);

            let mut curr_bits_index = curr_bits_len;

            while number > 0 {
                curr_bits_index -= 1;

                let fib_number = fibbonaci.get(curr_bits_index);

                if fib_number <= number {
                    number -= fib_number;
                    curr_bits.set_bit(curr_bits_index, Bit::ONE);
                }
            }

            bits.append_bits(&curr_bits);
        }

        bits
    }
}

/// State machine keeping track of fibbonaci decoder decoding state.
enum DecodingState {
    Empty,
    InsideNumber(usize, usize, bool), // number, index and was last bit one
}

impl DecodingState {
    pub fn next(
        self,
        bit: Bit,
        numbers: &mut Vec<usize>,
        fibbonaci: &mut Fibbonaci,
    ) -> DecodingState {
        match (self, bit) {
            (DecodingState::Empty, bit) => DecodingState::start_number(bit),
            (DecodingState::InsideNumber(number, _, true), Bit::ONE) => {
                DecodingState::end_number(number, numbers)
            }
            (DecodingState::InsideNumber(number, index, _), Bit::ONE) => {
                DecodingState::add_from_fibbonaci(number, index, fibbonaci)
            }
            (DecodingState::InsideNumber(number, index, _), Bit::ZERO) => {
                DecodingState::skip_zero(number, index)
            }
        }
    }

    fn start_number(bit: Bit) -> DecodingState {
        match bit {
            Bit::ONE => DecodingState::InsideNumber(1, 1, true),
            Bit::ZERO => DecodingState::InsideNumber(0, 1, false),
        }
    }

    fn end_number(number: usize, numbers: &mut Vec<usize>) -> DecodingState {
        numbers.push(number);
        DecodingState::Empty
    }

    fn add_from_fibbonaci(
        mut number: usize,
        index: usize,
        fibbonaci: &mut Fibbonaci,
    ) -> DecodingState {
        number += fibbonaci.get(index);
        DecodingState::InsideNumber(number, index + 1, true)
    }

    fn skip_zero(number: usize, index: usize) -> DecodingState {
        DecodingState::InsideNumber(number, index + 1, false)
    }
}

/// Fibbonaci decoder.
pub struct FibbonaciDecoder;

impl NumberDecoder for FibbonaciDecoder {
    fn decode(bits: &Bits) -> Vec<usize> {
        let mut numbers = vec![];
        let mut fibbonaci = Fibbonaci::new();

        let mut decoding_state = DecodingState::Empty;

        for bit in bits.iter() {
            decoding_state = decoding_state.next(bit, &mut numbers, &mut fibbonaci);
        }

        numbers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibbonaci_works() {
        let mut fibbonaci = Fibbonaci::new();

        let index = fibbonaci.find_greater_index(137);

        assert_eq!(10, index);
    }

    #[test]
    fn encode_works() {
        let number = [137];

        let bits = FibbonaciEncoder::encode(&number);

        assert_eq!([0b10000101, 0b01100000], bits.get_bits());
    }

    #[test]
    fn decode_number_works() {
        let numbers = [1, 2, 257, 259, 258, 2];

        let encoded = FibbonaciEncoder::encode(&numbers);
        let decoded = FibbonaciDecoder::decode(&encoded);

        assert_eq!(vec![1, 2, 257, 259, 258, 2], decoded);
    }
}
