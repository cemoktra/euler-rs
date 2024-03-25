pub fn is_palindrome(mut n: usize) -> bool {
    if n != 0 && n % 10 == 0 {
        return false;
    }

    let input = n;
    let mut reversed = 0;
    while n > 0 {
        reversed *= 10;
        reversed += n % 10;
        n /= 10;
    }
    input == reversed
}

pub fn is_binary_palindrome(mut n: usize) -> bool {
    let input = n;
    let mut reversed = 0usize;
    while n > 0 {
        reversed <<= 1;
        if n & 1 > 0 {
            reversed ^= 1;
        }
        n >>= 1;
    }
    input == reversed
}

#[cfg(test)]
mod test {
    #[test]
    fn test_palindrome() {
        assert!(super::is_palindrome(9009));
        assert!(super::is_palindrome(906609));
        assert!(!super::is_palindrome(90660));
    }

    #[test]
    fn test_binary_palindrome() {
        assert!(super::is_binary_palindrome(585));
        assert!(!super::is_binary_palindrome(123));
    }
}
