pub enum Bit {
    ONE,
    ZERO,
}

pub struct Bits {
    storage: Vec<u8>,
    size: usize,
}

impl Bits {
    pub fn new() -> Bits {
        Bits {
            storage: vec![],
            size: 0,
        }
    }

    pub fn push_bit(&mut self, bit: Bit) {
        let bit_position = self.size % 8;

        if bit_position == 0 {
            self.storage.push(0);
        }

        if let Bit::ONE = bit {
            let byte = self.size / 8;
            let mask = 1 << (7 - bit_position);

            self.storage[byte] |= mask;
        }

        self.size += 1;
    }

    pub fn append_bits(&mut self, bits: &Bits) {
        for i in 0..bits.size {
            self.push_bit(bits.get_bit(i));
        }
    }

    fn get_bit(&self, index: usize) -> Bit {
        if index > self.size - 1 {
            panic!("index too big");
        }

        let byte = index / 8;
        let bit = index % 8;

        if self.storage[byte] & 1 << (7 - bit) > 0 {
            Bit::ONE
        } else {
            Bit::ZERO
        }
    }

    pub fn get_bits(&self) -> &[u8] {
        &self.storage
    }
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
}
