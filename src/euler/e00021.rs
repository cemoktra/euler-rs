use crate::core::primes::{prime_factors, Divisors};

pub fn solve(n: usize) -> usize {
    let mut sum = 0;
    for i in 3..n {
        let n_div = prime_factors(i).divisors();
        let n_sum = n_div.iter().rev().skip(1).sum::<usize>();
        let x_div = prime_factors(n_sum).divisors();
        let x_sum = x_div.iter().rev().skip(1).sum::<usize>();

        if x_sum == i && i != n_sum {
            sum += i;
        }
    }

    sum
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(31626, super::solve(10000));
    }
}
