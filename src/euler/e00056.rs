use crate::core::digits::Digits;
use num_bigint::BigUint;

pub fn solve(limit: usize) -> usize {
    let mut max = 0;

    for i in 1..limit {
        let i_big: BigUint = i.into();
        let mut power = i_big.clone();

        for _ in 1..limit {
            power *= i_big.clone();

            let sum = Digits::new(power.clone()).fold(0, |sum, d| sum + (d as usize));
            if sum > max {
                max = sum
            }
        }
    }

    max
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(972, super::solve(100));
    }
}
