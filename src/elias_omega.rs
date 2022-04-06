const USIZE_HALF: usize = std::usize::MAX - (std::usize::MAX >> 1);

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
        let number = 0b10111;

        let bit_len = get_usize_bit_len(number);

        assert_eq!(5, bit_len);
    }
}
