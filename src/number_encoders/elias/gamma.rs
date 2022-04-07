//! Things for implementing gamma variant of elias encoding and decoding.

use crate::bits::{self, Bit, Bits};
use crate::number_encoders::{NumberDecoder, NumberEncoder};

/// Gamma variant of elias encoder.
pub struct EliasGammaEncoder;

impl EliasGammaEncoder {
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

impl NumberEncoder for EliasGammaEncoder {
    fn encode(numbers: &[usize]) -> Bits {
        let mut bits = Bits::new();

        for &number in numbers {
            let mut all_bits = Vec::new();

            if number == 1 {
                bits.push_bit(Bit::ONE);
                continue;
            }

            EliasGammaEncoder::encode_number(number, &mut all_bits);
            EliasGammaEncoder::encode_zeros(number, &mut all_bits);

            for i in (0..all_bits.len()).rev() {
                bits.append_bits(&all_bits[i]);
            }
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
pub struct EliasGammaDecoder;

impl EliasGammaDecoder {
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

impl NumberDecoder for EliasGammaDecoder {
    fn decode(bits: &Bits) -> Vec<usize> {
        let mut numbers = vec![];

        let mut decoding_state = DecodingState::Empty;

        for bit in bits.iter() {
            decoding_state = match (decoding_state, bit) {
                (DecodingState::Empty, Bit::ONE) => EliasGammaDecoder::decode_one(&mut numbers),
                (DecodingState::Empty, Bit::ZERO) => EliasGammaDecoder::start_counting_zeros(),
                (DecodingState::CountingZeros(n), Bit::ZERO) => EliasGammaDecoder::count_zero(n),
                (DecodingState::CountingZeros(n), Bit::ONE) => {
                    EliasGammaDecoder::end_counting_zeros(n)
                }
                (DecodingState::InsideNumber(bits, len), bit) => {
                    EliasGammaDecoder::get_number_bit(bits, len, bit, &mut numbers)
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
    fn encode_works() {
        let number = [137];

        let bits = EliasGammaEncoder::encode(&number);

        assert_eq!([0b00000001, 0b00010010], bits.get_bits());
    }

    #[test]
    fn decode_number_works() {
        let numbers = [1, 2, 257, 259, 258, 2];

        let encoded = EliasGammaEncoder::encode(&numbers);
        let decoded = EliasGammaDecoder::decode(&encoded);

        assert_eq!(vec![1, 2, 257, 259, 258, 2], decoded);
    }
}
