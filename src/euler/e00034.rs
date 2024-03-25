use crate::core::digits::Digits;

pub fn solve() -> usize {
    const LIMIT: usize = 362880 * 9;
    let factorials = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    let mut total = 0;

    for i in 3..LIMIT {
        if i == Digits::new(i).map(|i| factorials[i as usize]).sum() {
            total += i;
        }
    }

    total
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(40730, super::solve());
    }
}
