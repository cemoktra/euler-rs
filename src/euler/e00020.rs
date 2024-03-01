use crate::core::ubig::UBig;

pub fn solve(n: usize) -> usize {
    let mut f: UBig<400> = [1].as_slice().into();
    for i in 2..=n {
        let n = i.into();
        f *= n;
    }
    f.digits().sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(27, super::solve(10));
        assert_eq!(648, super::solve(100));
    }
}
