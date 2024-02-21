pub fn solve(max_factor: usize) -> usize {
    let mut max = 0;
    let lower = max_factor / 10;

    for x in (lower..max_factor).rev() {
        if max > x * (max_factor - 1) {
            break;
        }
        for y in (x - 1..max_factor).rev() {
            let p = x * y;
            if p > max && is_palindrome(p) {
                max = p;
            }
        }
    }

    max
}

fn is_palindrome(mut n: usize) -> bool {
    if n != 0 && n % 10 == 0 {
        return false;
    }

    let input = n;
    let mut reversed = 0;
    while n > 0 {
        reversed *= 10;
        reversed += n % 10;
        n /= 10;
    }
    input == reversed
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert_eq!(9009, super::solve(100));
        assert_eq!(906609, super::solve(1000));
    }

    #[test]
    fn test_is_palindrome() {
        assert!(super::is_palindrome(9009));
        assert!(super::is_palindrome(906609));
        assert!(!super::is_palindrome(90660));
    }
}

// ├─ 13195         58.71 ns      │ 61.65 ns      │ 60.06 ns      │ 60.09 ns      │ 100     │ 3200
// ╰─ 600851475143  1.687 µs      │ 1.898 µs      │ 1.697 µs      │ 1.704 µs      │ 100     │ 100
