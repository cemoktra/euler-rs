use super::ubig::UBig;

trait FibonacciInit {
    fn one() -> Self;
    fn two() -> Self;
}

impl FibonacciInit for usize {
    fn one() -> Self {
        1
    }

    fn two() -> Self {
        2
    }
}

impl<const N: usize> FibonacciInit for UBig<N> {
    fn one() -> Self {
        1usize.into()
    }

    fn two() -> Self {
        2usize.into()
    }
}

pub struct Fibonacci<T> {
    current: T,
    next: T,
}

impl<T> Default for Fibonacci<T>
where
    T: FibonacciInit,
{
    fn default() -> Self {
        Self {
            current: T::one(),
            next: T::two(),
        }
    }
}

impl<T> Iterator for Fibonacci<T>
where
    T: Copy + std::ops::AddAssign,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next += current;

        Some(current)
    }
}
