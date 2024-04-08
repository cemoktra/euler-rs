use crate::core::primes::is_prime;

pub fn solve() -> usize {
    let mut n = 3;
    let mut primes = vec![1, 2];

    loop {
        if is_prime(n) {
            primes.push(n);
        } else if !primes.iter().any(|p| {
            let rest = (n - p) / 2;
            let root = (rest as f64).sqrt().round() as usize;
            let root_2 = root * root;

            root_2 == rest
        }) {
            return n;
        }

        n += 2;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(5_777, super::solve());
    }
}
