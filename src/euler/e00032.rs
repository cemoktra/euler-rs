use crate::core::digits::Digits;
use std::collections::BTreeSet;

pub fn solve() -> usize {
    fn digit_mask(n: usize) -> u16 {
        Digits::new(n).fold(0u16, |acc, d| {
            acc | {
                if d == 0 {
                    acc
                } else {
                    acc | (1 << (d - 1))
                }
            }
        })
    }

    const TARGET_MASK: u16 = 0b1_1111_1111;
    let mut products = BTreeSet::new();

    for product in 1234..9876 {
        let product_mask = digit_mask(product);
        for multiplier in 123..=product / 2 {
            if product % multiplier != 0 {
                continue;
            }
            let mut mask = product_mask ^ digit_mask(multiplier);
            mask ^= digit_mask(product / multiplier);
            if mask == TARGET_MASK {
                products.insert(product);
            }
        }
    }

    products.iter().sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(45228, super::solve());
    }
}
