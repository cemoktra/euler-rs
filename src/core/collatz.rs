pub struct CollatzIterator {
    current: usize,
}

impl CollatzIterator {
    pub fn new(start: usize) -> Self {
        Self { current: start }
    }
}

impl Iterator for CollatzIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        if current == 0 {
            return None;
        }

        if current % 2 == 0 {
            self.current = current >> 1;
        } else if current == 1 {
            self.current = 0;
        } else {
            self.current = 3 * current + 1;
        }

        Some(current)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(10, super::CollatzIterator::new(13).count());
        assert_eq!(525, super::CollatzIterator::new(837799).count());
    }
}
