pub fn solve(n: usize) -> usize {
    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    let mut counts = vec![0usize; 4096];
    counts[0] = 1;

    for c in coins {
        for i in 0..n {
            if c + i <= n {
                counts[c + i] += counts[i]
            }
        }
    }

    counts[n]
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(73682, super::solve(200));
    }
}
