use crate::core::ubig::UBig;

pub fn solve(n: usize) -> usize {
    let mut big: UBig<400> = [2].as_slice().into();

    for _ in 1..n {
        big = big + big;
    }

    big.digits().sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(26, super::solve(15));
        assert_eq!(1366, super::solve(1000));
    }
}
