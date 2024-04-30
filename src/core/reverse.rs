use super::digits::Digits;

pub trait ReverseNumber {
    fn reverse_number(self) -> Self;
}

impl ReverseNumber for u128 {
    fn reverse_number(self) -> Self {
        let digits = Digits::new(self).collect::<Vec<_>>();
        digits.iter().fold(0, |acc, d| acc * 10 + (*d as u128))
    }
}

#[cfg(test)]
mod test {
    use super::ReverseNumber;

    #[test]
    fn test_reverse_number() {
        assert_eq!(987654321u128, 1234567890u128.reverse_number());
    }
}
