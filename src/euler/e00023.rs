use crate::core::primes::{prime_factors, Divisors};

const LIMIT: usize = 28124;

pub fn solve() -> usize {
    let mut abundant = [false; LIMIT];

    for (n, item) in abundant.iter_mut().enumerate().take(LIMIT).skip(1) {
        let sum_divisors = prime_factors(n)
            .divisors()
            .iter()
            .rev()
            .skip(1)
            .sum::<usize>();
        if sum_divisors > n {
            *item = true;
        }
    }

    (1..LIMIT)
        .filter(|n| {
            let mut j = 1;
            let mut summable = true;
            let upper = n / 2 + 1;

            while summable && j < upper {
                if abundant[j] && abundant[n - j] {
                    summable = false;
                }
                j += 1;
            }
            summable
        })
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(4179871, super::solve());
    }
}
