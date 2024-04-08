pub struct Hexagonal {
    x: usize,
    y: usize,
}

impl Default for Hexagonal {
    fn default() -> Self {
        Self { x: 1, y: 5 }
    }
}

impl Iterator for Hexagonal {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.x;
        self.x += self.y;
        self.y += 4;
        Some(n)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_hexagonal() {
        let mut hexa = super::Hexagonal::default();
        assert_eq!(Some(1), hexa.next());
        assert_eq!(Some(6), hexa.next());
        assert_eq!(Some(15), hexa.next());
        assert_eq!(Some(28), hexa.next());
        assert_eq!(Some(45), hexa.next());
    }
}
