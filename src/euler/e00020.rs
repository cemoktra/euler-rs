use crate::core::digits::Digits;
use num_bigint::BigUint;

pub fn solve(n: usize) -> usize {
    let mut f: BigUint = 1u32.into();
    for i in 2..=n {
        let n: BigUint = i.into();
        f *= n;
    }
    Digits::new(f).map(|n| n as usize).sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(27, super::solve(10));
        assert_eq!(648, super::solve(100));
    }
}
