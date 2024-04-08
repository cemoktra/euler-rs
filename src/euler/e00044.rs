use crate::core::pentagonal::{is_pentagonal, Pentagonal};

pub fn solve() -> usize {
    for p_sum in Pentagonal::default() {
        for p_lower in Pentagonal::default() {
            let p_bigger = p_sum - p_lower;
            if p_bigger < p_lower {
                break;
            }
            let p_diff = p_bigger - p_lower;
            if is_pentagonal(p_diff) && is_pentagonal(p_bigger) {
                return p_diff;
            }
        }
    }

    0
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(5_482_660, super::solve());
    }
}
