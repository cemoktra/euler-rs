pub fn solve(max: usize) -> usize {
    fn sum_divisible(n: usize, target: usize) -> usize {
        let x = target / n;
        n * (x * (x + 1)) / 2
    }
    sum_divisible(3, max - 1) + sum_divisible(5, max - 1) - sum_divisible(15, max - 1)
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(23, super::solve(10));
        assert_eq!(233168, super::solve(1000));
    }
}
