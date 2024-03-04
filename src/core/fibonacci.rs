pub struct Fibonacci<T> {
    current: T,
    next: T,
}

impl<T> Default for Fibonacci<T>
where
    T: From<usize>,
{
    fn default() -> Self {
        Self {
            current: 1usize.into(),
            next: 2usize.into(),
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
