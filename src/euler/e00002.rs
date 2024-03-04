use crate::core::fibonacci::Fibonacci;

pub fn solve(max: usize) -> usize {
    Fibonacci::<usize>::default()
        .take_while(|n| *n < max)
        .filter(|n| n % 2 == 0)
        .sum()
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert_eq!(4613732, super::solve(4_000_000));
    }
}
