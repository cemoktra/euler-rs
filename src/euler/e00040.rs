use crate::core::digits::Digits;

pub fn solve() -> usize {
    let mut n = 1;
    let mut fractions_digits = Vec::<u8>::with_capacity(1_000_000);

    while fractions_digits.len() < 1_000_000 {
        let digits = Digits::new(n).collect::<Vec<_>>();
        fractions_digits.extend(digits.iter().rev());
        n += 1;
    }

    let d1 = fractions_digits[0] as usize;
    let d10 = fractions_digits[9] as usize;
    let d100 = fractions_digits[99] as usize;
    let d1000 = fractions_digits[999] as usize;
    let d10000 = fractions_digits[9999] as usize;
    let d100000 = fractions_digits[99999] as usize;
    let d1000000 = fractions_digits[999999] as usize;

    d1 * d10 * d100 * d1000 * d10000 * d100000 * d1000000
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(210, super::solve());
    }
}
