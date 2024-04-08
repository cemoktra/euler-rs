pub struct Pentagonal {
    x: usize,
    y: usize,
}

impl Default for Pentagonal {
    fn default() -> Self {
        Self { x: 1, y: 4 }
    }
}

impl Iterator for Pentagonal {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.x;
        self.x += self.y;
        self.y += 3;
        Some(n)
    }
}

pub fn is_pentagonal(n: usize) -> bool {
    // FIXME: use isqrt once stable
    let root = ((1 + 24 * n) as f64).sqrt().round() as usize;
    root * root == 1 + 24 * n && (1 + root) % 6 == 0
}

#[cfg(test)]
mod test {
    #[test]
    fn test_pentagonal() {
        let mut penta = super::Pentagonal::default();
        assert_eq!(Some(1), penta.next());
        assert_eq!(Some(5), penta.next());
        assert_eq!(Some(12), penta.next());
        assert_eq!(Some(22), penta.next());
        assert_eq!(Some(35), penta.next());

        assert!(super::is_pentagonal(5));
        assert!(super::is_pentagonal(12));
        assert!(super::is_pentagonal(22));
        assert!(!super::is_pentagonal(2));
        assert!(!super::is_pentagonal(15));
    }
}
