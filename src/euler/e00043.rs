use crate::core::permutations::Permutations;

pub fn solve() -> usize {
    let mut sum = 0;
    // the last 3 digits need to be divislbe by 17
    for last_three in (0..1000).step_by(17) {
        let d10 = last_three % 10;
        let d9 = last_three / 10 % 10;
        let d8 = last_three / 100;

        // must be different digits
        if d8 == d9 || d8 == d10 || d9 == d10 {
            continue;
        }

        let used_digits = (1 << d8) | (1 << d9) | (1 << d10);
        for d7 in 0..10 {
            // digit already used
            if (used_digits >> d7) & 1 == 1 {
                continue;
            }

            let used_digits = used_digits | (1 << d7);
            let last_four = last_three + d7 * 1000;
            if last_four / 10 % 13 != 0 {
                continue;
            }

            // remaining digits
            for perm in
                Permutations::new((0..10).filter(|&d| (used_digits >> d) & 1 == 0).collect())
            {
                let d3 = perm[2] as usize;
                let d4 = perm[3] as usize;
                let d5 = perm[4] as usize;
                let d6 = perm[5] as usize;

                // divisible by 2
                if d4 & 1 == 1 {
                    continue;
                }
                // divisible by 3
                if (d3 + d4 + d5) % 3 != 0 {
                    continue;
                }
                // divisible by 5
                if d6 % 5 != 0 {
                    continue;
                }

                let d567 = d5 * 100 + d6 * 10 + d7;
                if d567 % 7 != 0 {
                    continue;
                }
                let d678 = d6 * 100 + d7 * 10 + d8;
                if d678 % 11 != 0 {
                    continue;
                }

                let n = perm.iter().fold(0, |acc, d| acc * 10 + (*d as usize)) * 10000 + last_four;
                sum += n;
            }
        }
    }

    sum
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(16_695_334_890, super::solve());
    }
}
