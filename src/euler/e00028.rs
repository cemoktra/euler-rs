// derived from
// top right => n ^ 2
// top left => n ^ 2 - (n - 1)
// bottom left => n ^ 2 - 2(n - 1)
// bottom right => n ^ 2 - 3(n - 1)
//
// n² + n² - (n - 1) + n² - 2(n - 1) + n² - 3(n - 1)
// 4n² - 6(n-1)
pub fn solve(n: usize) -> usize {
    (3..=n)
        .step_by(2)
        .map(|n| 4 * n.pow(2) - 6 * (n - 1))
        .sum::<usize>()
        + 1
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(101, super::solve(5));
        assert_eq!(669_171_001, super::solve(1_001));
    }
}
