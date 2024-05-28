use num_bigint::BigUint;

pub mod collatz;
pub mod digits;
pub mod fibonacci;
pub mod hexagonal;
pub mod lychrel;
pub mod palindrome;
pub mod pentagonal;
pub mod permutations;
pub mod poker;
pub mod primes;
pub mod reverse;
pub mod triangle;

pub trait Ten {
    fn ten() -> Self;
}

impl Ten for i32 {
    fn ten() -> Self {
        10
    }
}
impl Ten for usize {
    fn ten() -> Self {
        10
    }
}
impl Ten for u128 {
    fn ten() -> Self {
        10
    }
}

impl Ten for BigUint {
    fn ten() -> Self {
        10u32.into()
    }
}
