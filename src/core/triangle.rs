pub struct TriangleNumbers {
    next: usize,
    step: usize,
}

impl Default for TriangleNumbers {
    fn default() -> Self {
        Self { next: 1, step: 2 }
    }
}

impl Iterator for TriangleNumbers {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;
        self.next += self.step;
        self.step += 1;

        Some(current)
    }
}
