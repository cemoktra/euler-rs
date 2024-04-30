use crate::core::palindrome::{is_binary_palindrome, IsPalindrome};

pub fn solve(n: usize) -> usize {
    (1..n)
        .filter(|n| n.is_palindrome() && is_binary_palindrome(*n))
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(872187, super::solve(1_000_000));
    }
}
