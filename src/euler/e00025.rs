use crate::core::digits::Digits;
use crate::core::fibonacci::Fibonacci;
use num_bigint::BigUint;

pub fn solve(digits: usize) -> usize {
    for (i, f) in Fibonacci::<BigUint>::default().enumerate() {
        if Digits::new(f).count() >= digits {
            // our sequence skips the first 1 and we start counting at 1
            return i + 2;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(4782, super::solve(1_000));
    }
}
