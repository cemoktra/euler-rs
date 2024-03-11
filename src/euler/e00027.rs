use crate::core::primes::{is_prime, Primes};

pub fn solve(max: i64) -> i64 {
    let mut n_max = 0;
    let mut product = 0;

    let max_usize = max.try_into().expect("positive max");
    for b in Primes::default().take_while(|b| *b < max_usize) {
        let b_i64 = b as i64;
        let b_f64 = b as f64;

        // FIXME: use isqrt once stable
        let limit = 2 * b_f64.sqrt().ceil() as i64;

        for a in -limit..limit {
            let mut n = 0;

            loop {
                let v = n * n + a * n + b_i64;
                if v < 0 || !is_prime(v.try_into().expect("expect to be positive")) {
                    break;
                }
                n += 1;
            }

            if n > n_max {
                n_max = n;
                product = a * b_i64;
            }
        }
    }

    product
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(-59231, super::solve(1_000));
    }
}
