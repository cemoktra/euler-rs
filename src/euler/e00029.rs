use crate::core::primes::factors;
use std::collections::BTreeSet;

pub fn solve(n: usize) -> usize {
    let mut unique = BTreeSet::new();
    for base in 2..=n {
        let prime_factors = factors(base);
        for power in 2..=n {
            unique.insert(
                prime_factors
                    .iter()
                    .map(|(a, b)| (*a, *b * power))
                    .collect::<Vec<_>>(),
            );
        }
    }

    unique.len()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(15, super::solve(5));
        assert_eq!(9183, super::solve(100));
    }
}
