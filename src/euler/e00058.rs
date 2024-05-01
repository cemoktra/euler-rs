use crate::core::primes::is_prime;

pub fn solve() -> usize {
    let mut primes = 0;
    let mut count = 1;
    let mut side = 3;

    loop {
        let bottom_right = side * side;
        let bottom_left = bottom_right - (side - 1);
        let top_left = bottom_left - (side - 1);
        let top_right = top_left - (side - 1);

        count += 4;
        if is_prime(top_right) {
            primes += 1;
        }

        if is_prime(top_left) {
            primes += 1;
        }

        if is_prime(bottom_left) {
            primes += 1;
        }

        if primes * 10 < count {
            return side;
        }

        side += 2;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(26_241, super::solve());
    }
}
