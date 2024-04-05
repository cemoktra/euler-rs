use crate::core::digits::Digits;

pub fn solve() -> usize {
    const ALL_DIGITS: usize = 0b1111111110;
    let mut max = 0;

    for i in 1..100_000 {
        let mut digits = 0;
        let mut n = 1;
        let mut products = vec![];

        let result = 'outer: loop {
            let product = n * i;

            let mut digit_vec = Digits::new(product).collect::<Vec<_>>();
            digit_vec.reverse();

            let mut product_digits = 0;
            for d in digit_vec.iter() {
                let update = product_digits | (1 << d);
                if update == product_digits {
                    break 'outer None;
                }
                product_digits = update;
            }

            if digits & product_digits > 0 {
                break None;
            } else {
                digits |= product_digits;
                products.extend(digit_vec);

                if digits == ALL_DIGITS {
                    break Some(products.iter().fold(0, |acc, d| acc * 10 + (*d as usize)));
                }
            }

            n += 1;
        };

        if let Some(result) = result {
            if result > max {
                max = result;
            }
        }
    }

    max
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(932718654, super::solve());
    }
}
