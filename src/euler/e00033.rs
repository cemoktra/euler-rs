use crate::core::primes::prime_factors;

/// (10x + y) / (10y + z) = x / z
/// z = (10xy) / (9x + y) where x < z
pub fn solve() -> usize {
    let mut res_numer = 0;
    let mut res_denom = 1;
    for x in 1..=9 {
        for y in 1..=9 {
            let numer = 10 * x * y;
            let denom = 9 * x + y;
            let z = numer / denom;
            let rest = numer % denom;
            if rest == 0 && x < z {
                res_numer += x;
                res_denom *= z;
            }
        }
    }

    for (base, power) in prime_factors(res_numer) {
        res_denom /= base.pow(power as u32);
    }

    res_denom
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(100, super::solve());
    }
}
