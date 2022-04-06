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
        self.size += 1;

        let bit_position = self.size % 8;

        if bit_position == 1 {
            self.storage.push(0);
        }

        if let Bit::ONE = bit {
            let byte = self.size / 8;
            let mask = 1 << (8 - bit_position);

            self.storage[byte] |= mask;
        }
    }
}
