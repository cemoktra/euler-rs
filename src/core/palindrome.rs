pub trait IsPalindrome:
    Sized
    + std::cmp::Eq
    + std::cmp::PartialOrd
    + std::ops::Rem<Output = Self>
    + std::ops::DivAssign
    + std::ops::AddAssign
    + std::ops::MulAssign
    + Copy
{
    fn zero() -> Self;
    fn ten() -> Self;

    fn is_palindrome(&self) -> bool {
        let zero = Self::zero();
        let ten = Self::ten();
        let mut n = *self;

        if n != zero && n % ten == zero {
            return false;
        }

        let input = n;
        let mut reversed = zero;
        while n > zero {
            reversed *= ten;
            reversed += n % ten;
            n /= ten;
        }
        input == reversed
    }
}

impl IsPalindrome for u128 {
    fn zero() -> Self {
        0
    }

    fn ten() -> Self {
        10
    }
}

impl IsPalindrome for usize {
    fn zero() -> Self {
        0
    }

    fn ten() -> Self {
        10
    }
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
    use super::IsPalindrome;

    #[test]
    fn test_palindrome() {
        assert!(9009usize.is_palindrome());
        assert!(906609usize.is_palindrome());
        assert!(!90660usize.is_palindrome());
    }

    #[test]
    fn test_binary_palindrome() {
        assert!(super::is_binary_palindrome(585));
        assert!(!super::is_binary_palindrome(123));
    }
}
