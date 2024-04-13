use std::collections::HashSet;

use crate::core::{digits::Digits, primes::Primes};

pub fn solve() -> usize {
    let primes = Primes::default()
        .skip_while(|p| *p < 100_000)
        .take_while(|p| *p < 1_000_000)
        .collect::<Vec<_>>();

    let mut digits = HashSet::new();
    let mut group = HashSet::new();
    let prime_set: HashSet<usize> = primes.clone().into_iter().collect();

    for p in &primes {
        digits.clear();

        let mut prime_digits = Digits::new(*p).collect::<Vec<_>>();
        prime_digits.reverse();

        for d in &prime_digits {
            digits.insert(*d);
        }

        for d in &digits {
            group.clear();
            group.insert(*p);

            for r in 0..10 {
                if r == *d {
                    continue;
                }

                let x = prime_digits
                    .iter()
                    .map(|p| if p == d { r } else { *p })
                    .fold(0, |acc, d| acc * 10 + (d as usize));

                if x > 100_000 && prime_set.contains(&x) {
                    group.insert(x);
                }
            }

            if group.len() == 8 {
                return group.into_iter().min().unwrap_or_default();
            }
        }
    }

    0
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(121313, super::solve());
    }
}
