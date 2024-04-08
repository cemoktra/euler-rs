use crate::core::primes::prime_factors;
use std::collections::HashSet;

pub fn solve(uniques: usize) -> usize {
    let mut unique_count = 0;
    let mut n = 1;
    let mut factors = HashSet::new();

    loop {
        let f = prime_factors(n);

        if f.len() != uniques {
            unique_count = 0;
            n += 1;
            continue;
        }

        if factors.contains(&f) {
            unique_count = 0;
        } else {
            unique_count += 1;
            factors.insert(f);
        }

        if unique_count == uniques {
            return n - uniques + 1;
        }

        n += 1;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(14, super::solve(2));
        assert_eq!(644, super::solve(3));
        assert_eq!(134043, super::solve(4));
    }
}
