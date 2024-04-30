use crate::core::ubig::UBig;

pub fn solve(limit: usize) -> usize {
    let mut numer = UBig::<400>::from(3);
    let mut denom = UBig::<400>::from(2);
    let mut count = 0;

    for _ in 1..=limit {
        let tmp = numer + UBig::<400>::from(2) * denom;
        denom = numer + denom;
        numer = tmp;

        let numer_digits = numer.digits().skip_while(|p| **p == 0).count();
        let denom_digits = denom.digits().skip_while(|p| **p == 0).count();

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
