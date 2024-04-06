use std::collections::HashSet;

use crate::core::triangle::TriangleNumbers;

pub fn solve(words: &[&str]) -> usize {
    let triangle_numbers = TriangleNumbers::default()
        .take_while(|n| *n < 192)
        .collect::<HashSet<_>>();

    words
        .iter()
        .map(|word| {
            let word_value = word
                .bytes()
                .map(|b| b - 64)
                .fold(0, |acc, d| acc + (d as usize));
            word_value
        })
        .filter(|word_value| triangle_numbers.contains(word_value))
        .count()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let file_content = std::fs::read_to_string("./data/e00042.txt").unwrap();
        let words = file_content
            .split(',')
            .map(|word| word.trim_end_matches('\n').trim_matches('"'))
            .collect::<Vec<_>>();

        assert_eq!(162, super::solve(words.as_slice()));
    }
}
