use crate::core::collatz::CollatzIterator;
use std::collections::HashMap;

pub fn solve(n: usize) -> usize {
    let mut cache = HashMap::<usize, usize>::new();
    cache.insert(1, 1);
    cache.insert(2, 2);

    for i in 3..n {
        let (count, c) = CollatzIterator::new(i)
            .enumerate()
            .find(|(_, n)| n < &i)
            .expect("");

        let value = cache.get(&c).expect("has been calculated");
        cache.insert(i, count + value);
    }

    *cache
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .expect("contains values")
        .0
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(837799, super::solve(1_000_000));
    }
}
