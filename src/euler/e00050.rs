use crate::core::primes::Primes;

pub fn solve(target: usize) -> usize {
    let primes = Primes::default()
        .take_while(|p| *p < target)
        .collect::<Vec<_>>();
    let mut max_chain = primes.len();

    let mut sum = 0;
    for (i, p) in primes.iter().enumerate() {
        sum += *p;
        if sum >= target {
            max_chain = i;
            break;
        }
    }

    let mut length = max_chain;
    loop {
        let bound = max_chain - length;
        for start in (0..=bound).rev() {
            let sum = primes[start..start + length].iter().sum();
            if sum < target && primes.iter().any(|p| *p == sum) {
                return sum;
            }
        }

        length -= 1;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(41, super::solve(100));
        assert_eq!(953, super::solve(1_000));
        assert_eq!(997651, super::solve(1_000_000));
    }
}
