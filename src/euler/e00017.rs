fn word_count(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 3,                            // one
        2 => 3,                            // two
        3 => 5,                            // three
        4 => 4,                            // four
        5 => 4,                            // five
        6 => 3,                            // six
        7 => 5,                            // seven
        8 => 5,                            // eight
        9 => 4,                            // nine
        10 => 3,                           // ten
        11 => 6,                           // eleven
        12 => 6,                           // twelve
        13 => 8,                           // thirteen
        14 => 8,                           // fourteen
        15 => 7,                           // fifteen
        16 => 7,                           // sixteen
        17 => 9,                           // seventeen
        18 => 8,                           // eighteen
        19 => 8,                           // nineteen
        20..=29 => 6 + word_count(n % 10), // twenty + n % 10
        30..=39 => 6 + word_count(n % 10), // thirty + n % 10
        40..=49 => 5 + word_count(n % 10), // forty + n % 10
        50..=59 => 5 + word_count(n % 10), // fifty + n % 10
        60..=69 => 5 + word_count(n % 10), // sixty + n % 10
        70..=79 => 7 + word_count(n % 10), // seventy + n % 10
        80..=89 => 6 + word_count(n % 10), // eighty + n % 10
        90..=99 => 6 + word_count(n % 10), // ninety + n % 10
        100..=999 => {
            // decimal + hundred (7)
            let hundred = word_count(n / 100) + 7;
            if n % 100 == 0 {
                hundred
            } else {
                // + and (3) + n % 100
                hundred + 3 + word_count(n % 100)
            }
        }
        1000 => 11, // one thousand
        _ => panic!("1000 is the limit"),
    }
}

pub fn solve(n: usize) -> usize {
    (1..=n).map(word_count).sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(19, super::solve(5));
        assert_eq!(21124, super::solve(1000));
    }
}
