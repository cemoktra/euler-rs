use crate::core::digits::Digits;
use crate::core::primes::Primes;
use std::collections::HashSet;

pub fn solve(n: usize) -> usize {
    let primes = Primes::default()
        .take_while(|p| *p < n)
        .collect::<HashSet<_>>();
    let mut sum = 0;

    for p in &primes {
        if *p < 10 {
            continue;
        }
        let mut digits = Digits::new(*p).collect::<Vec<_>>();
        digits.reverse();

        let truncate = (1..digits.len()).all(|i| {
            let l = digits[i..]
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, d)| acc + (*d as usize) * 10usize.pow(i as u32));
            let r = digits[0..i]
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, d)| acc + (*d as usize) * 10usize.pow(i as u32));

            primes.contains(&l) && primes.contains(&r)
        });

        if truncate {
            sum += p;
        }
    }

    sum
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(748317, super::solve(1_000_000));
    }
}
