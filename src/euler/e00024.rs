use crate::core::permutations::Permutations;

pub fn solve(n: usize) -> usize {
    let digits = Permutations::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        .nth(n - 1)
        .expect("it exists");

    let mut res = 0usize;

    for (p, n) in digits.iter().rev().enumerate() {
        res += (*n as usize) * 10u64.pow(p as u32) as usize;
    }

    res
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(2783915460, super::solve(1_000_000));
    }
}
