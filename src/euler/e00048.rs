fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus
    }
    result
}

pub fn solve(max: u128) -> u128 {
    let modulo = 10000000000;
    (1..=max).map(|i| mod_pow(i, i, modulo)).sum::<u128>() % modulo
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(9110846700, super::solve(1000));
    }
}
