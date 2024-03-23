pub fn solve(n: usize) -> usize {
    let factors = crate::core::primes::prime_factors(n);
    factors.into_keys().max().unwrap_or_default()
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert_eq!(6857, super::solve(600_851_475_143));
    }
}

// ├─ 13195         58.71 ns      │ 61.65 ns      │ 60.06 ns      │ 60.09 ns      │ 100     │ 3200
// ╰─ 600851475143  1.687 µs      │ 1.898 µs      │ 1.697 µs      │ 1.704 µs      │ 100     │ 100
