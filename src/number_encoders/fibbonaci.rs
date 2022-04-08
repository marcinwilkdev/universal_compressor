//! Things for implementing fibbonaci encoding.

use crate::bits::{self, Bit, Bits};
use crate::number_encoders::{NumberDecoder, NumberEncoder};

pub struct Fibbonaci {
    cache: Vec<usize>,
}

impl Fibbonaci {
    pub fn new() -> Fibbonaci {
        Fibbonaci { cache: vec![1, 2] }
    }

    pub fn find_greater_index(&mut self, number: usize) -> usize { // index and value
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

/// Gamma variant of elias encoder.
pub struct FibbonaciEncoder;

impl FibbonaciEncoder {
    fn encode_number(number: usize, all_bits: &mut Vec<Bits>) {
        let number_bits: Bits = number.into();
        all_bits.push(number_bits);
    }

    fn encode_zeros(number_len: usize, all_bits: &mut Vec<Bits>) {
        let mut last_bits = Bits::new();
        for _ in 0..bits::get_usize_bit_len(number_len) - 1 {
            last_bits.push_bit(Bit::ZERO);
        }
        all_bits.push(last_bits);
    }
}

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

/// State machine keeping track of elias gamma decoding state.
enum DecodingState {
    Empty,
    InsideNumber(Bits, usize),
    CountingZeros(usize),
}

/// Gamma variant of elias decoder.
pub struct FibbonaciDecoder;

impl FibbonaciDecoder {
    fn decode_one(numbers: &mut Vec<usize>) -> DecodingState {
        numbers.push(1);
        DecodingState::Empty
    }

    fn start_counting_zeros() -> DecodingState {
        DecodingState::CountingZeros(1)
    }

    fn count_zero(n: usize) -> DecodingState {
        DecodingState::CountingZeros(n + 1)
    }

    fn end_counting_zeros(n: usize) -> DecodingState {
        DecodingState::InsideNumber(1.into(), n)
    }

    fn get_number_bit(
        mut bits: Bits,
        len: usize,
        bit: Bit,
        numbers: &mut Vec<usize>,
    ) -> DecodingState {
        bits.push_bit(bit);

        if len == 1 {
            numbers.push(bits.into());
            DecodingState::Empty
        } else {
            DecodingState::InsideNumber(bits, len - 1)
        }
    }
}

impl NumberDecoder for FibbonaciDecoder {
    fn decode(bits: &Bits) -> Vec<usize> {
        let mut numbers = vec![];

        let mut decoding_state = DecodingState::Empty;

        for bit in bits.iter() {
            decoding_state = match (decoding_state, bit) {
                (DecodingState::Empty, Bit::ONE) => FibbonaciDecoder::decode_one(&mut numbers),
                (DecodingState::Empty, Bit::ZERO) => FibbonaciDecoder::start_counting_zeros(),
                (DecodingState::CountingZeros(n), Bit::ZERO) => FibbonaciDecoder::count_zero(n),
                (DecodingState::CountingZeros(n), Bit::ONE) => {
                    FibbonaciDecoder::end_counting_zeros(n)
                }
                (DecodingState::InsideNumber(bits, len), bit) => {
                    FibbonaciDecoder::get_number_bit(bits, len, bit, &mut numbers)
                }
            }
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

    // #[test]
    // fn decode_number_works() {
    //     let numbers = [1, 2, 257, 259, 258, 2];

    //     let encoded = FibbonaciEncoder::encode(&numbers);
    //     let decoded = FibbonaciDecoder::decode(&encoded);

    //     assert_eq!(vec![1, 2, 257, 259, 258, 2], decoded);
    // }
}
