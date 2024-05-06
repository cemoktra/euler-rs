use std::collections::HashSet;

use itertools::Itertools;

use crate::core::{
    digits::Digits,
    primes::{CachedIsPrime, Primes},
};

fn is_prime_pair(p1: usize, p2: usize, is_prime_cache: &mut CachedIsPrime) -> bool {
    let d1 = Digits::new(p1).collect_vec();
    let d2 = Digits::new(p2).collect_vec();

    let x1 = d1
        .iter()
        .rev()
        .chain(d2.iter().rev())
        .fold(0, |acc, d| acc * 10 + (*d as usize));

    let x2 = d2
        .iter()
        .rev()
        .chain(d1.iter().rev())
        .fold(0, |acc, d| acc * 10 + (*d as usize));

    is_prime_cache.is_prime(x1) && is_prime_cache.is_prime(x2)
}

pub fn solve(n: usize) -> usize {
    let mut is_prime_cache = CachedIsPrime::default();
    let mut prime_pairs = HashSet::new();
    let primes = Primes::default()
        .take_while(|p| *p < 10_000)
        .collect::<Vec<_>>();
    let mut min = usize::MAX;

    for p1 in &primes {
        // it wont get any better from here
        if *p1 > min {
            break;
        }

        let mut current_pairs = vec![];

        for p2 in &primes {
            if *p1 == *p2 {
                break;
            }

            if is_prime_pair(*p1, *p2, &mut is_prime_cache) {
                prime_pairs.insert((*p1, *p2));
                current_pairs.push(*p2);
            }
        }

        for combination in current_pairs.iter().combinations(n - 1) {
            if combination
                .iter()
                .combinations(2)
                .all(|p| prime_pairs.contains(&(**p[1], **p[0])))
            {
                let sum = *p1 + combination.iter().cloned().sum::<usize>();
                if sum < min {
                    min = sum;
                }
            }
        }
    }

    min
}

#[cfg(test)]
mod test {
    #[test]
    fn test_is_prime_pair() {
        let mut is_prime_cache = super::CachedIsPrime::default();
        assert!(super::is_prime_pair(3, 7, &mut is_prime_cache));
        assert!(super::is_prime_pair(3, 109, &mut is_prime_cache));
        assert!(super::is_prime_pair(3, 673, &mut is_prime_cache));
        assert!(super::is_prime_pair(7, 109, &mut is_prime_cache));
        assert!(super::is_prime_pair(7, 673, &mut is_prime_cache));
        assert!(super::is_prime_pair(109, 673, &mut is_prime_cache));
    }

    #[test]
    fn test() {
        assert_eq!(10, super::solve(2));
        assert_eq!(107, super::solve(3));
        assert_eq!(792, super::solve(4));
        assert_eq!(26033, super::solve(5));
    }
}
