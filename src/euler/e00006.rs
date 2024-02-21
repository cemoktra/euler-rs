pub fn solve(max: usize) -> usize {
    let (sum, sum_squares) = (1..=max).fold((0, 0), |acc, i| (acc.0 + i, acc.1 + i * i));
    (sum * sum) - sum_squares
}
#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert_eq!(2640, super::solve(10));
        assert_eq!(25164150, super::solve(100));
    }
}
