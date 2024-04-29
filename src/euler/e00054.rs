use crate::core::poker::Hand;

pub fn solve(hand_pairs: &[(Hand, Hand)]) -> usize {
    let mut count = 0;

    for (h1, h2) in hand_pairs {
        if h1.rank() > h2.rank() {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let file_content = std::fs::read_to_string("./data/e00054.txt").unwrap();
        let hand_pairs = file_content
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let hand1 = (&line[0..14]).into();
                let hand2 = (&line[15..]).into();
                (hand1, hand2)
            })
            .collect::<Vec<_>>();

        assert_eq!(376, super::solve(hand_pairs.as_slice()));
    }
}
