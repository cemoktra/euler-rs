pub struct Digits {
    current: usize,
}

impl Digits {
    pub fn new(n: usize) -> Self {
        Self { current: n }
    }
}

impl Iterator for Digits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 0 {
            return None;
        }
        let d = self.current % 10;
        self.current /= 10;

        Some(d as u8)
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
