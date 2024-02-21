use crate::core::primes::Primes;

pub fn solve(max: usize) -> usize {
    Primes::default().take_while(|n| *n < max).sum()
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert_eq!(17, super::solve(10));
        assert_eq!(142913828922, super::solve(2000000));
    }
}
