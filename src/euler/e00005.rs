use crate::core::primes::Primes;

pub fn solve(max: usize) -> usize {
    Primes::default()
        .take_while(|p| *p <= max)
        .map(|p| {
            let x = (max as f64).powf(1.0 / p as f64);
            let x = x.floor() as u32;
            p.pow(x)
        })
        .product()
}
#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert_eq!(2520, super::solve(10));
        assert_eq!(232792560, super::solve(20));
    }
}
