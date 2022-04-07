const USIZE_HALF: usize = std::usize::MAX - (std::usize::MAX >> 1);

#[derive(Debug)]
pub enum Bit {
    ONE,
    ZERO,
}

#[derive(Clone)]
pub struct Bits {
    bytes: Vec<u8>,
    size: usize,
}

impl Bits {
    pub fn new() -> Bits {
        Bits {
            bytes: vec![],
            size: 0,
        }
    }

    pub fn from_vec(size: usize, bytes: Vec<u8>) -> Bits {
        Bits { size, bytes }
    }

    pub fn iter(&self) -> BitsIterator {
        BitsIterator {
            bits: self,
            index: 0,
        }
    }

    pub fn push_bit(&mut self, bit: Bit) {
        let bit_position = self.size % 8;

        if bit_position == 0 {
            self.bytes.push(0);
        }

        if let Bit::ONE = bit {
            let byte = self.size / 8;
            let mask = Bits::create_mask(bit_position);

            self.bytes[byte] |= mask;
        }

        self.size += 1;
    }

    pub fn append_bits(&mut self, bits: &Bits) {
        for i in 0..bits.size {
            self.push_bit(bits.get_bit(i));
        }
    }

    pub fn get_bits(&self) -> &[u8] {
        &self.bytes
    }

    pub fn shift_left_and_shrink_size(&mut self) {
        for i in 0..self.size - 1 {
            self.set_bit(i, self.get_bit(i+1));
        }

        self.set_bit(self.size - 1, Bit::ZERO);

        self.size -= 1;
    }

    fn get_bit(&self, index: usize) -> Bit {
        if index > self.size - 1 {
            panic!("index too big");
        }

        let byte = index / 8;
        let bit_position = index % 8;
        let mask = Bits::create_mask(bit_position);

        if self.bytes[byte] & mask > 0 {
            Bit::ONE
        } else {
            Bit::ZERO
        }
    }

    fn set_bit(&mut self, index: usize, bit: Bit) {
        if index > self.size - 1 {
            panic!("index too big");
        }

        let byte_index = index / 8;
        let bit_position = index % 8;

        let mut byte = 0;

        for i in 0..8 {
            if i != bit_position {
                byte |= self.bytes[byte_index] & Bits::create_mask(i);
            } else {
                if let Bit::ONE = bit {
                    byte |= Bits::create_mask(i);
                }
            }
        }

        self.bytes[byte_index] = byte;
    }

    fn create_mask(bit_position: usize) -> u8 {
        if bit_position > 7 {
            panic!("too big bit position");
        }

        1 << (7 - bit_position)
    }
}

pub struct BitsIterator<'a> {
    bits: &'a Bits,
    index: usize,
}

impl Iterator for BitsIterator<'_> {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.bits.size {
            let bit = self.bits.get_bit(self.index);
            self.index += 1;
            Some(bit)
        } else {
            None
        }
    }
}

impl From<usize> for Bits {
    fn from(number: usize) -> Self {
        let mut start_mask = get_usize_bit_len(number) - 1;
        let mut bits = Bits::new();

        loop {
            if number & 1 << start_mask > 0 {
                bits.push_bit(Bit::ONE);
            } else {
                bits.push_bit(Bit::ZERO);
            }

            if start_mask == 0 {
                break;
            }

            start_mask -= 1;
        }

        return bits;
    }
}

impl From<Bits> for usize {
    fn from(bits: Bits) -> Self {
        let mut mask = bits.size;
        let mut number = 0;

        while mask > 0 {
            if let Bit::ONE = bits.get_bit(bits.size - mask) {
                number += 1 << (mask - 1);
            }

            mask -= 1;
        }

        number
    }
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
    fn bits_working() {
        let mut bits = Bits::new();

        for i in 0..9 {
            if i % 2 == 0 {
                bits.push_bit(Bit::ONE);
            } else {
                bits.push_bit(Bit::ZERO);
            }
        }

        assert_eq!([0b10101010, 0b10000000], bits.get_bits());
    }

    #[test]
    fn append_bits_working() {
        let mut bits = Bits::new();

        for i in 0..5 {
            if i % 2 == 0 {
                bits.push_bit(Bit::ONE);
            } else {
                bits.push_bit(Bit::ZERO);
            }
        }

        let other_bits = Bits::new();

        for i in 0..4 {
            if i % 2 == 0 {
                bits.push_bit(Bit::ONE);
            } else {
                bits.push_bit(Bit::ZERO);
            }
        }

        bits.append_bits(&other_bits);

        assert_eq!([0b10101101, 0b00000000], bits.get_bits());
    }

    #[test]
    fn from_usize_works() {
        let number = 137;

        let bits: Bits = number.into();

        assert_eq!([0b10001001], bits.get_bits());
    }

    #[test]
    fn usize_from_bits_works() {
        let number = 137;
        let bits: Bits = number.into();
        let other_number: usize = bits.into();

        assert_eq!(number, other_number);
    }

    #[test]
    fn get_usize_bit_len_works() {
        let number = 137;

        let bit_len = get_usize_bit_len(number) - 1;
        let another_bit_len = get_usize_bit_len(bit_len) - 1;

        assert_eq!(0b111, bit_len);
        assert_eq!(0b10, another_bit_len);
    }

    #[test]
    fn set_bit_works() {
        let number = 0b11010110;

        let mut bits: Bits = number.into();
        bits.set_bit(0, Bit::ZERO);

        assert_eq!(0b01010110, bits.get_bits()[0]);
    }

    #[test]
    fn shift_left_works() {
        let number = 0b11010110;

        let mut bits: Bits = number.into();
        bits.shift_left_and_shrink_size();

        assert_eq!(0b10101100, bits.get_bits()[0]);
    }
}
