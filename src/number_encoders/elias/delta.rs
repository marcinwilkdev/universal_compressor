//! Things for implementing delta variant of elias encoding and decoding.

use crate::bits::{self, Bit, Bits};
use crate::number_encoders::{NumberDecoder, NumberEncoder};

/// Delta variant of elias encoder.
pub struct EliasDeltaEncoder;

impl EliasDeltaEncoder {
    fn encode_number(number: usize, all_bits: &mut Vec<Bits>) {
        let mut number_bits: Bits = number.into();
        number_bits.shift_left_and_shrink_size();
        all_bits.push(number_bits);
    }

    fn encode_number_len(number_len: usize, all_bits: &mut Vec<Bits>) {
        all_bits.push(number_len.into());
    }

    fn encode_zeros(number_len: usize, all_bits: &mut Vec<Bits>) {
        let mut last_bits = Bits::new();
        for _ in 0..bits::get_usize_bit_len(number_len) - 1 {
            last_bits.push_bit(Bit::ZERO);
        }
        all_bits.push(last_bits);
    }
}

impl NumberEncoder for EliasDeltaEncoder {
    fn encode(numbers: &[usize]) -> Bits {
        let mut bits = Bits::new();

        for &number in numbers {
            let mut all_bits = Vec::new();

            if number == 1 {
                bits.push_bit(Bit::ONE);
                continue;
            }

            let number_len = bits::get_usize_bit_len(number);

            EliasDeltaEncoder::encode_number(number, &mut all_bits);
            EliasDeltaEncoder::encode_number_len(number_len, &mut all_bits);
            EliasDeltaEncoder::encode_zeros(number_len, &mut all_bits);

            for i in (0..all_bits.len()).rev() {
                bits.append_bits(&all_bits[i]);
            }
        }

        bits
    }
}

/// State machine keeping track of elias omega decoding state.
enum DecodingState {
    Empty,
    InsideLen(Bits, usize),
    InsideNumber(Bits, usize),
    CountingZeros(usize),
}

/// Omega variant of elias decoder.
pub struct EliasDeltaDecoder;

impl EliasDeltaDecoder {
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
        DecodingState::InsideLen(1.into(), n)
    }

    fn get_len_bit(mut bits: Bits, len: usize, bit: Bit) -> DecodingState {
        bits.push_bit(bit);

        if len == 1 {
            let len: usize = bits.into();
            DecodingState::InsideNumber(1.into(), len - 1)
        } else {
            DecodingState::InsideLen(bits, len - 1)
        }
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

impl NumberDecoder for EliasDeltaDecoder {
    fn decode(bits: &Bits) -> Vec<usize> {
        let mut numbers = vec![];

        let mut decoding_state = DecodingState::Empty;

        for bit in bits.iter() {
            decoding_state = match (decoding_state, bit) {
                (DecodingState::Empty, Bit::ONE) => EliasDeltaDecoder::decode_one(&mut numbers),
                (DecodingState::Empty, Bit::ZERO) => EliasDeltaDecoder::start_counting_zeros(),
                (DecodingState::CountingZeros(n), Bit::ZERO) => EliasDeltaDecoder::count_zero(n),
                (DecodingState::CountingZeros(n), Bit::ONE) => {
                    EliasDeltaDecoder::end_counting_zeros(n)
                }
                (DecodingState::InsideLen(bits, len), bit) => {
                    EliasDeltaDecoder::get_len_bit(bits, len, bit)
                }
                (DecodingState::InsideNumber(bits, len), bit) => {
                    EliasDeltaDecoder::get_number_bit(bits, len, bit, &mut numbers)
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

        let bits = EliasDeltaEncoder::encode(&number);

        assert_eq!([0b00010000, 0b00100100], bits.get_bits());
    }

    #[test]
    fn decode_number_works() {
        let numbers = [1, 2, 257, 259, 258, 2];

        let encoded = EliasDeltaEncoder::encode(&numbers);
        let decoded = EliasDeltaDecoder::decode(&encoded);

        assert_eq!(vec![1, 2, 257, 259, 258, 2], decoded);
    }
}
