use crate::core::primes::Primes;
pub fn solve(n: usize) -> usize {
    Primes::default().nth(n - 1).unwrap_or_default()
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert_eq!(13, super::solve(6));
        assert_eq!(104743, super::solve(10001));
    }
}
