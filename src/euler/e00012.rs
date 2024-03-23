use crate::core::primes::prime_factors;

pub fn solve(max: usize) -> usize {
    let mut triangular = 0;

    for i in 1.. {
        triangular += i;
        let factors = prime_factors(triangular);
        if factors.values().map(|f| f + 1).product::<usize>() > max {
            return triangular;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(28, super::solve(5));
        assert_eq!(76576500, super::solve(500));
    }
}
