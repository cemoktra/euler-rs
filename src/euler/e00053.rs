pub fn solve(target: usize) -> usize {
    let mut count = 0;

    let mut c = vec![vec![0; 101]; 101];

    for (n, c) in c.iter_mut().enumerate() {
        c[0] = 1;
        c[n] = 1;
    }

    for n in 1..=100 {
        for k in 1..n {
            let mut sum = c[n - 1][k - 1] + c[n - 1][k];
            if sum > target {
                sum = target + 1;
                count += 1;
            }
            c[n][k] = sum
        }
    }

    count
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(4075, super::solve(1_000_000));
    }
}
