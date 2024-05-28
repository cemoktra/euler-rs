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
    T: Clone + num_traits::CheckedAdd,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.clone();
        self.current = self.next.clone();
        match self.next.checked_add(&current) {
            None => None,
            Some(next) => {
                self.next = next;
                Some(current)
            }
        }
    }
}
