use crate::core::Ten;
use std::fmt::Debug;

pub struct Digits<T> {
    current: T,
}

impl<T> Digits<T> {
    pub fn new(n: T) -> Self {
        Self { current: n }
    }
}

impl<T> Iterator for Digits<T>
where
    T: num_traits::Zero,
    T: std::ops::Rem + std::ops::DivAssign,
    T: Clone,
    T: Ten,
    <T as std::ops::Rem>::Output: TryInto<u8>,
    <<T as std::ops::Rem>::Output as TryInto<u8>>::Error: Debug,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_zero() {
            return None;
        }

        let ten = T::ten();
        let d = self.current.clone() % ten.clone();
        self.current /= ten;

        Some(d.try_into().expect("a digit is always a u8"))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_digit_iterator() {
        let mut digits = super::Digits::new(123456);
        assert_eq!(digits.next(), Some(6));
        assert_eq!(digits.next(), Some(5));
        assert_eq!(digits.next(), Some(4));
        assert_eq!(digits.next(), Some(3));
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), None);
    }
}
