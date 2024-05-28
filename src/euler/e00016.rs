use crate::core::digits::Digits;
use num_bigint::BigUint;

pub fn solve(n: usize) -> usize {
    let mut big: BigUint = 2u32.into();

    for _ in 1..n {
        big = big.clone() + big;
    }

    Digits::new(big).map(|n| n as usize).sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(26, super::solve(15));
        assert_eq!(1366, super::solve(1000));
    }
}
