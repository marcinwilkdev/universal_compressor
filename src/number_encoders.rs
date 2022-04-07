//! Traits and structs for numbers encoding and decoding.

pub mod elias;
pub mod fibbonaci;

use crate::Bits;

/// Trait used for encoding `usize` numbers.
pub trait NumberEncoder {
    fn encode(numbers: &[usize]) -> Bits;
}

/// Trait used for decoding `usize` numbers.
pub trait NumberDecoder {
    fn decode(bits: &Bits) -> Vec<usize>;
}
