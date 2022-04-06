use crate::bits::{Bit, Bits};
use std::collections::VecDeque;

const USIZE_HALF: usize = std::usize::MAX - (std::usize::MAX >> 1);

pub fn encode_number_and_append(mut number: usize, bits: &mut Bits) {
    let mut all_bits: VecDeque<Bits> = VecDeque::new();

    if number == 1 {
        bits.push_bit(crate::bits::Bit::ZERO);
        return;
    }

    all_bits.push_front(0.into());

    while get_usize_bit_len(number) > 1 {
        all_bits.push_front(number.into());
        number = get_usize_bit_len(number) - 1;
    }

    for b in all_bits {
        bits.append_bits(&b);
    }
}

pub fn decode_number(bytes: &Bits) -> Vec<usize> {
    let mut numbers = vec![];

    let mut curr_bit = 0;
    let mut curr_number_bits: Bits = 1.into();
    let mut just_started = true;

    for bit in bytes.iter() {
        if curr_bit == 0 {
            match bit {
                Bit::ONE => {
                    curr_bit = curr_number_bits.clone().into();
                    curr_number_bits = 1.into();
                }
                Bit::ZERO => {
                    if just_started {
                        numbers.push(1);
                    } else {
                        let curr_number: usize = curr_number_bits.clone().into();
                        numbers.push(curr_number);

                        curr_number_bits = 1.into();
                    }
                }
            }
            just_started = true;
        } else {
            curr_number_bits.push_bit(bit);
            curr_bit -= 1;
            just_started = false;
        }
    }

    numbers
}

pub fn get_usize_bit_len(number: usize) -> usize {
    let mut size = 1;
    let mut mask = 1;

    while mask < USIZE_HALF && 2 * mask <= number {
        mask *= 2;
        size += 1;
    }

    return size;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_usize_bit_len_works() {
        let number = 137;

        let bit_len = get_usize_bit_len(number) - 1;
        let another_bit_len = get_usize_bit_len(bit_len) - 1;

        assert_eq!(0b111, bit_len);
        assert_eq!(0b10, another_bit_len);
    }

    #[test]
    fn encode_number_and_append_works() {
        let mut bits = Bits::new();

        let number = 137;

        encode_number_and_append(number, &mut bits);

        assert_eq!([0b10111100, 0b01001000], bits.get_bits());
    }

    #[test]
    fn decode_number_works() {
        let mut bits = Bits::new();

        let numbers = [1, 2, 257, 259, 258, 2];

        for number in numbers {
            encode_number_and_append(number, &mut bits);
        }

        let numbers = decode_number(&bits);

        assert_eq!(vec![1, 2, 257, 259, 258, 2], numbers);
    }
}
