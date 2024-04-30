use super::{palindrome::IsPalindrome, reverse::ReverseNumber};

const MAX_ITERATIONS: u128 = 50;

pub fn is_lychrel(mut n: u128) -> bool {
    for _ in 0..MAX_ITERATIONS {
        let x1 = n;
        let x2 = n.reverse_number();

        n = x1 + x2;

        if n.is_palindrome() {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    #[test]
    fn test_lychrel() {
        assert!(super::is_lychrel(47));
        assert!(super::is_lychrel(349));

        assert!(!super::is_lychrel(196));
    }
}
