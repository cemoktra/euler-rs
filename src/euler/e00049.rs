use crate::core::{
    digits::Digits,
    primes::{is_prime, Primes},
};

pub fn solve() -> usize {
    for p in Primes::default()
        .skip_while(|p| *p < 1488)
        .take_while(|p| *p < 3340)
    {
        let p1 = p + 3330;
        if !is_prime(p1) {
            continue;
        }
        let p2 = p1 + 3330;
        if !is_prime(p2) {
            continue;
        }

        let mask = Digits::new(p).fold(0, |acc, d| acc | (1 << d));
        let mask1 = Digits::new(p1).fold(0, |acc, d| acc | (1 << d));

        if mask != mask1 {
            continue;
        }

        let mask2 = Digits::new(p2).fold(0, |acc, d| acc | (1 << d));

        if mask != mask2 {
            continue;
        }

        return p * 100_000_000 + p1 * 10_000 + p2;
    }

    0
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(296_962_999_629, super::solve());
    }
}
