use crate::core::fibonacci::Fibonacci;
use crate::core::ubig::UBig;

pub fn solve(digits: usize) -> usize {
    for (i, f) in Fibonacci::<UBig<1000>>::default().enumerate() {
        if f.digits().skip_while(|d| **d == 0).count() >= digits {
            // our sequence skips the first 1 and we start counting at 1
            return i + 2;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(4782, super::solve(1_000));
    }
}
