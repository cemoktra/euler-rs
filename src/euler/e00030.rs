use crate::core::digits::Digits;

pub fn solve(n: u32) -> usize {
    let mut max_digits = 2;
    loop {
        let x = max_digits * 9usize.pow(n);
        let y = 10usize.pow((max_digits - 1) as u32);
        if y > x {
            max_digits -= 1;
            break;
        }
        max_digits += 1;
    }
    let max = max_digits * 9usize.pow(n);

    let mut result = 0;
    for j in 2..max {
        let sum: usize = Digits::new(j).map(|d| (d as usize).pow(n)).sum();
        if sum == j {
            result += j
        }
    }

    result
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(19316, super::solve(4));
        assert_eq!(443839, super::solve(5));
    }
}
