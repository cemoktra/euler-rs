use crate::core::digits::Digits;
use crate::core::primes::Primes;
use std::collections::HashSet;

pub fn solve(n: usize) -> usize {
    let primes = Primes::default()
        .take_while(|p| *p < n)
        .collect::<HashSet<_>>();
    let mut result = 4; // 2, 3, 5, 7

    for p in &primes {
        if *p < 10 {
            continue;
        }
        let mut digits = Digits::new(*p).collect::<Vec<_>>();
        digits.reverse();
        if digits.iter().any(|d| d % 2 == 0 || *d == 5) {
            continue;
        }

        let circular_prime = (0..digits.len()).all(|_| {
            digits.rotate_left(1);
            let r = digits
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, d)| acc + (*d as usize) * 10usize.pow(i as u32));
            primes.contains(&r)
        });

        if circular_prime {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(13, super::solve(100));
        assert_eq!(55, super::solve(1_000_000));
    }
}
