use crate::core::digits::Digits;
use num_bigint::BigUint;

pub fn solve(limit: usize) -> usize {
    let mut numer = BigUint::from(3u32);
    let mut denom = BigUint::from(2u32);
    let mut count = 0;

    for _ in 1..=limit {
        let tmp = numer.clone() + BigUint::from(2u32) * denom.clone();
        denom = numer.clone() + denom.clone();
        numer = tmp;

        let numer_digits = Digits::new(numer.clone()).count();
        let denom_digits = Digits::new(denom.clone()).count();

        if numer_digits > denom_digits {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(153, super::solve(1000));
    }
}
