use crate::core::{digits::Digits, primes::Primes};

pub fn solve() -> usize {
    const PANDIGITAL_1: usize = 0b10;
    const PANDIGITAL_2: usize = 0b110;
    const PANDIGITAL_3: usize = 0b1110;
    const PANDIGITAL_4: usize = 0b11110;
    const PANDIGITAL_5: usize = 0b111110;
    const PANDIGITAL_6: usize = 0b1111110;
    const PANDIGITAL_7: usize = 0b11111110;
    const PANDIGITAL_8: usize = 0b111111110;
    const PANDIGITAL_9: usize = 0b1111111110;

    let mut max = 0;
    for p in Primes::default()
        .skip_while(|p| *p < 10_000)
        .take_while(|p| *p < 9_999_999)
    {
        let digits = Digits::new(p);

        let (mask, count) = digits.fold((0, 0), |(mask, count), d| (mask | (1 << d), count + 1));
        let is_pandigital = match count {
            1 => PANDIGITAL_1 == mask,
            2 => PANDIGITAL_2 == mask,
            3 => PANDIGITAL_3 == mask,
            4 => PANDIGITAL_4 == mask,
            5 => PANDIGITAL_5 == mask,
            6 => PANDIGITAL_6 == mask,
            7 => PANDIGITAL_7 == mask,
            8 => PANDIGITAL_8 == mask,
            9 => PANDIGITAL_9 == mask,
            _ => false,
        };

        if is_pandigital {
            max = p;
        }
    }
    max
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(7652413, super::solve());
    }
}
