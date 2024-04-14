use crate::core::digits::Digits;

pub fn solve() -> usize {
    const MAX_FACTOR: usize = 17;

    for power in 2..10 {
        let min = 10usize.pow(power);
        let max = MAX_FACTOR * 10usize.pow(power - 1);

        for n in min..max {
            let mask = Digits::new(n).fold(0, |acc, d| acc | (1 << d));
            if (2..=6)
                .map(|x| Digits::new(n * x).fold(0, |acc, d| acc | (1 << d)))
                .all(|m| m == mask)
            {
                return n;
            }
        }
    }

    0
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(142857, super::solve());
    }
}
