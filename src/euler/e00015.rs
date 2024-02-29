pub fn solve(n: u128) -> u128 {
    let x = (n + 1..=2 * n).product::<u128>();
    let y = (1..=n).product::<u128>();
    x / y
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(6, super::solve(2));
        assert_eq!(137846528820u128, super::solve(20));
    }
}
