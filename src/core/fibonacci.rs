pub struct Fibonacci {
    current: usize,
    next: usize,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self {
            current: 1,
            next: 2,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next += current;

        Some(current)
    }
}
