pub fn solve(n: usize) -> usize {
    let mut outcome = [0usize; 400];
    outcome[399] = 2;

    for _ in 1..n {
        let mut carry = 0;
        outcome.iter_mut().rev().for_each(|value| {
            let sum = *value + *value + carry;
            *value = sum % 10;
            carry = sum / 10;
        });
    }

    outcome.iter().sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(26, super::solve(15));
        assert_eq!(1366, super::solve(1000));
    }
}
