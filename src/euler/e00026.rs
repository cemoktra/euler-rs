use crate::core::primes::{is_prime, Primes};
// TODO: impl pow and mod for UBig
use num_bigint::BigUint;

pub fn solve(max: usize) -> usize {
    Primes::default()
        .take_while(|p| *p < max)
        .filter(|p| is_prime((p - 1) / 2))
        .filter(|p| {
            let base_10 = BigUint::from(10usize)
                .pow((p - 1).try_into().expect("smaller than 1000"))
                - BigUint::from(1usize);
            base_10 % p == BigUint::from(0usize)
        })
        .max()
        .unwrap_or_default()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(983, super::solve(1_000));
    }
}
