use crate::core::{hexagonal::Hexagonal, pentagonal::is_pentagonal};

pub fn solve() -> usize {
    for h in Hexagonal::default().skip(143) {
        if is_pentagonal(h) {
            return h;
        }
    }

    0
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(1_533_776_805, super::solve());
    }
}
