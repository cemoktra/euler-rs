use crate::core::palindrome::{is_binary_palindrome, is_palindrome};

pub fn solve(n: usize) -> usize {
    (1..n)
        .filter(|n| is_palindrome(*n) && is_binary_palindrome(*n))
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(0, super::solve(1_000_000));
    }
}
