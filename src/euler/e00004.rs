use crate::core::palindrome::IsPalindrome;

pub fn solve(max_factor: usize) -> usize {
    let mut max = 0;
    let lower = max_factor / 10;

    for x in (lower..max_factor).rev() {
        if max > x * (max_factor - 1) {
            break;
        }
        for y in (x - 1..max_factor).rev() {
            let p = x * y;
            if p > max && p.is_palindrome() {
                max = p;
            }
        }
    }

    max
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert_eq!(9009, super::solve(100));
        assert_eq!(906609, super::solve(1000));
    }
}
