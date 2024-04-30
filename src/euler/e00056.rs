use crate::core::ubig::UBig;

pub fn solve(limit: usize) -> usize {
    let mut max = 0;

    for i in 1..limit {
        let i_big: UBig<200> = i.into();
        let mut power = i_big;

        for _ in 1..limit {
            power *= i_big;

            let sum = power.digits().fold(0, |sum, d| sum + *d);
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
