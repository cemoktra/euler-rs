pub fn solve(target: i64) -> i64 {
    let half_target = target / 2;
    let third_target = target / 3;

    (2..=third_target)
        .rev()
        .filter_map(|a| {
            let numerator = target * (a - half_target);
            let denominator = a - 1000;

            let b = numerator / denominator;
            if b <= a {
                return None;
            }

            let c = target - a - b;
            if c <= b {
                return None;
            }

            if numerator % denominator != 0 {
                return None;
            }
            Some(a * b * c)
        })
        .next()
        .unwrap_or_else(|| panic!("a solution exists"))
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(31875000, super::solve(1000));
    }
}
