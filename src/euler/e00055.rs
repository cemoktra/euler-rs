use crate::core::lychrel::is_lychrel;

pub fn solve(limit: u128) -> usize {
    let mut count = 0;

    for i in 1..limit {
        if is_lychrel(i) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(249, super::solve(10_000));
    }
}
