//! Things for implementing omega variant of elias encoding and decoding.

use crate::bits::{self, Bit, Bits};
use crate::number_encoders::{NumberDecoder, NumberEncoder};

/// Omega variant of elias encoder.
pub struct EliasOmegaEncoder;

impl NumberEncoder for EliasOmegaEncoder {
    fn encode(numbers: &[usize]) -> Bits {
        let mut bits = Bits::new();

        for &(mut number) in numbers {
            let mut all_bits = Vec::new();

            if number == 1 {
                bits.push_bit(Bit::ZERO);
                continue;
            }

            all_bits.push(0.into());

            while bits::get_usize_bit_len(number) > 1 {
                all_bits.push(number.into());
                number = bits::get_usize_bit_len(number) - 1;
            }

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
    InsideWord(Bits, usize),
}

/// Omega variant of elias decoder.
pub struct EliasOmegaDecoder;

impl EliasOmegaDecoder {
    fn decode_zero(numbers: &mut Vec<usize>) -> DecodingState {
        numbers.push(1);
        DecodingState::Empty
    }

    fn start_decoding_number() -> DecodingState {
        DecodingState::InsideWord(1.into(), 1)
    }

    fn end_decoding_number(numbers: &mut Vec<usize>, bits: &Bits) -> DecodingState {
        numbers.push(bits.to_owned().into());
        DecodingState::Empty
    }

    fn next_step(bits: &Bits) -> DecodingState {
        DecodingState::InsideWord(1.into(), bits.to_owned().into())
    }

    fn next_bit(mut bits: Bits, curr_bit: usize, bit: Bit) -> DecodingState {
        bits.push_bit(bit);
        DecodingState::InsideWord(bits, curr_bit - 1)
    }
}

impl NumberDecoder for EliasOmegaDecoder {
    fn decode(bits: &Bits) -> Vec<usize> {
        let mut numbers = vec![];

        let mut decoding_state = DecodingState::Empty;

        for bit in bits.iter() {
            decoding_state = match (decoding_state, bit) {
                (DecodingState::Empty, Bit::ZERO) => EliasOmegaDecoder::decode_zero(&mut numbers),
                (DecodingState::Empty, Bit::ONE) => EliasOmegaDecoder::start_decoding_number(),
                (DecodingState::InsideWord(bits, 0), Bit::ZERO) => {
                    EliasOmegaDecoder::end_decoding_number(&mut numbers, &bits)
                }
                (DecodingState::InsideWord(bits, 0), Bit::ONE) => {
                    EliasOmegaDecoder::next_step(&bits)
                }
                (DecodingState::InsideWord(bits, curr_bit), bit) => {
                    EliasOmegaDecoder::next_bit(bits, curr_bit, bit)
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

        let bits = EliasOmegaEncoder::encode(&number);

        assert_eq!([0b10111100, 0b01001000], bits.get_bits());
    }

    #[test]
    fn decode_number_works() {
        let numbers = [1, 2, 257, 259, 258, 2];

        let encoded = EliasOmegaEncoder::encode(&numbers);
        let decoded = EliasOmegaDecoder::decode(&encoded);

        assert_eq!(vec![1, 2, 257, 259, 258, 2], decoded);
    }
}
