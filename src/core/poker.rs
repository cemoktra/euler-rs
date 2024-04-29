use std::fmt::Display;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Club => write!(f, "C"),
            Suit::Spade => write!(f, "S"),
            Suit::Heart => write!(f, "H"),
            Suit::Diamond => write!(f, "D"),
        }
    }
}

impl From<char> for Suit {
    fn from(value: char) -> Self {
        match value.to_ascii_lowercase() {
            'd' => Self::Diamond,
            'h' => Self::Heart,
            'c' => Self::Club,
            's' => Self::Spade,
            _ => panic!("unknown suit: {value}"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
#[repr(u64)]
pub enum Value {
    Two = 1 << 0,
    Three = 1 << 1,
    Four = 1 << 2,
    Five = 1 << 3,
    Six = 1 << 4,
    Seven = 1 << 5,
    Eight = 1 << 6,
    Nine = 1 << 7,
    Ten = 1 << 8,
    Jack = 1 << 9,
    Queen = 1 << 10,
    King = 1 << 11,
    Ace = 1 << 12,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Two => write!(f, "2"),
            Value::Three => write!(f, "3"),
            Value::Four => write!(f, "4"),
            Value::Five => write!(f, "5"),
            Value::Six => write!(f, "6"),
            Value::Seven => write!(f, "7"),
            Value::Eight => write!(f, "8"),
            Value::Nine => write!(f, "9"),
            Value::Ten => write!(f, "T"),
            Value::Jack => write!(f, "J"),
            Value::Queen => write!(f, "Q"),
            Value::King => write!(f, "K"),
            Value::Ace => write!(f, "A"),
        }
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        match value.to_ascii_lowercase() {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            't' => Self::Ten,
            'j' => Self::Jack,
            'q' => Self::Queen,
            'k' => Self::King,
            'a' => Self::Ace,
            _ => panic!("unknown value: {value}"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, Hash)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Self { value, suit }
    }

    pub fn value_mask(&self) -> u64 {
        self.value as u64
    }
}

impl FromIterator<char> for Card {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut iter = iter.into_iter();

        let value_char = iter.next().expect("a card has a value");
        let suit_char = iter.next().expect("a card has a suit");

        Self {
            value: value_char.into(),
            suit: suit_char.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Hand([Card; 5]);

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4]
        )
    }
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        Self(cards)
    }

    pub fn value_mask(&self) -> u64 {
        self.0.iter().fold(0u64, |set, c| {
            let rank = c.value as u64;
            set | rank
        })
    }

    pub fn suit_mask(&self) -> u64 {
        self.0.iter().fold(0, |set, c| {
            let rank = c.suit as u8;
            set | (1 << rank)
        })
    }

    pub fn straight(&self) -> Option<u64> {
        const ACE_AS_1: u64 = 0b1_0000_0000_1111;

        let mask = self.value_mask();

        let trailing_zeros = mask.trailing_zeros();
        let trailing_ones = (mask >> trailing_zeros).trailing_ones();

        if trailing_ones == 5 {
            Some((trailing_zeros + 2).into())
        } else if ACE_AS_1 == mask {
            Some(1)
        } else {
            None
        }
    }

    pub fn flush(&self) -> Option<u64> {
        if self.suit_mask().count_ones() == 1 {
            Some(self.0[4].value as u64)
        } else {
            None
        }
    }

    pub fn value_count(&self) -> [u8; 13] {
        let mut count = [0; 13];

        for card in &self.0 {
            let card_mask = card.value_mask();
            for (i, c) in count.iter_mut().enumerate() {
                if card_mask & (1 << i) > 0 {
                    *c += 1;
                }
            }
        }

        count
    }

    pub fn rank(&self) -> Rank {
        let flush = self.flush();
        let straight = self.straight();
        let values = self.value_mask();

        match (flush, straight) {
            (Some(flush), Some(straight)) => {
                if flush == Value::Ace as u64 {
                    Rank::RoyalFlush
                } else {
                    Rank::StraightFlush(straight)
                }
            }
            (Some(flush), None) => Rank::Flush(flush),
            (None, Some(straight)) => Rank::Straight(straight),
            (None, None) => {
                let value_count = self.value_count();

                for i in (0..13).rev() {
                    if value_count[i] == 4 {
                        return Rank::FourOfAKind(1 << i);
                    }
                }

                for i in (0..13).rev() {
                    if value_count[i] == 3 {
                        for count in value_count {
                            if count == 2 {
                                return Rank::FullHouse(1 << i);
                            }
                        }

                        let remaining_cards = values ^ (1 << i);
                        return Rank::ThreeOfAKind(1 << (i + 13) | remaining_cards);
                    }
                }

                for i in (0..13).rev() {
                    if value_count[i] == 2 {
                        for (j, count) in value_count.iter().enumerate() {
                            if i != j && *count == 2 {
                                let remaining_card = values ^ (1 << i | 1 << j);
                                return Rank::TwoPair(
                                    1 << (i + 26) | 1 << (j + 13) | remaining_card,
                                );
                            }
                        }
                        let remaining_cards = values ^ (1 << i);
                        return Rank::OnePair(1 << (i + 13) | remaining_cards);
                    }
                }

                Rank::HighCard(values)
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Rank {
    HighCard(u64),
    OnePair(u64),
    TwoPair(u64),
    ThreeOfAKind(u64),
    Straight(u64),
    Flush(u64),
    FullHouse(u64),
    FourOfAKind(u64),
    StraightFlush(u64),
    RoyalFlush,
}
impl Rank {
    pub fn value(&self) -> u64 {
        match self {
            Rank::HighCard(value) => *value,
            Rank::OnePair(value) => *value,
            Rank::TwoPair(value) => *value,
            Rank::ThreeOfAKind(value) => (1 << 40) | *value,
            Rank::Straight(value) => (1 << 41) | *value,
            Rank::Flush(value) => (1 << 42) | *value,
            Rank::FullHouse(value) => (1 << 43) | *value,
            Rank::FourOfAKind(value) => (1 << 44) | *value,
            Rank::StraightFlush(value) => (1 << 45) | *value,
            Rank::RoyalFlush => 1 << 46,
        }
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut cards = value
            .split_whitespace()
            .map(|s| Card::from_iter(s.chars()))
            .collect::<Vec<_>>();
        cards.sort();

        Self(cards.try_into().expect("five cards"))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_hand() {
        let hand: super::Hand = "AS KD 3D JD 8H".into();
        assert_eq!(
            hand,
            super::Hand::new([
                super::Card::new(super::Value::Three, super::Suit::Diamond),
                super::Card::new(super::Value::Eight, super::Suit::Heart),
                super::Card::new(super::Value::Jack, super::Suit::Diamond),
                super::Card::new(super::Value::King, super::Suit::Diamond),
                super::Card::new(super::Value::Ace, super::Suit::Spade),
            ])
        );

        let hand: super::Hand = "5H 5C 6S 7S KD".into();
        assert_eq!(
            hand,
            super::Hand::new([
                super::Card::new(super::Value::Five, super::Suit::Club),
                super::Card::new(super::Value::Five, super::Suit::Heart),
                super::Card::new(super::Value::Six, super::Suit::Spade),
                super::Card::new(super::Value::Seven, super::Suit::Spade),
                super::Card::new(super::Value::King, super::Suit::Diamond),
            ])
        );
    }

    #[test]
    fn test_straight() {
        let hand = super::Hand::new([
            super::Card::new(super::Value::Two, super::Suit::Club),
            super::Card::new(super::Value::Three, super::Suit::Heart),
            super::Card::new(super::Value::Four, super::Suit::Spade),
            super::Card::new(super::Value::Five, super::Suit::Spade),
            super::Card::new(super::Value::Ace, super::Suit::Diamond),
        ]);
        assert_eq!(Some(1), hand.straight());

        let hand = super::Hand::new([
            super::Card::new(super::Value::Seven, super::Suit::Club),
            super::Card::new(super::Value::Eight, super::Suit::Heart),
            super::Card::new(super::Value::Nine, super::Suit::Spade),
            super::Card::new(super::Value::Ten, super::Suit::Spade),
            super::Card::new(super::Value::Jack, super::Suit::Diamond),
        ]);
        assert_eq!(Some(7), hand.straight());

        let hand = super::Hand::new([
            super::Card::new(super::Value::Five, super::Suit::Club),
            super::Card::new(super::Value::Five, super::Suit::Heart),
            super::Card::new(super::Value::Six, super::Suit::Spade),
            super::Card::new(super::Value::Seven, super::Suit::Spade),
            super::Card::new(super::Value::King, super::Suit::Diamond),
        ]);
        assert!(hand.straight().is_none());
    }

    #[test]
    fn test_flush() {
        let hand = super::Hand::new([
            super::Card::new(super::Value::Five, super::Suit::Club),
            super::Card::new(super::Value::Eight, super::Suit::Club),
            super::Card::new(super::Value::Nine, super::Suit::Club),
            super::Card::new(super::Value::Jack, super::Suit::Club),
            super::Card::new(super::Value::King, super::Suit::Club),
        ]);
        assert_eq!(Some(super::Value::King as u64), hand.flush());

        let hand = super::Hand::new([
            super::Card::new(super::Value::Five, super::Suit::Diamond),
            super::Card::new(super::Value::Eight, super::Suit::Club),
            super::Card::new(super::Value::Ten, super::Suit::Club),
            super::Card::new(super::Value::Jack, super::Suit::Club),
            super::Card::new(super::Value::King, super::Suit::Club),
        ]);
        assert!(hand.flush().is_none());
    }

    #[test]
    fn test_hand_rank() {
        let royal_flush = super::Hand::new([
            super::Card::new(super::Value::Ten, super::Suit::Club),
            super::Card::new(super::Value::Jack, super::Suit::Club),
            super::Card::new(super::Value::Queen, super::Suit::Club),
            super::Card::new(super::Value::King, super::Suit::Club),
            super::Card::new(super::Value::Ace, super::Suit::Club),
        ]);
        assert_eq!(super::Rank::RoyalFlush, royal_flush.rank());

        let straight_flush = super::Hand::new([
            super::Card::new(super::Value::Nine, super::Suit::Club),
            super::Card::new(super::Value::Ten, super::Suit::Club),
            super::Card::new(super::Value::Jack, super::Suit::Club),
            super::Card::new(super::Value::Queen, super::Suit::Club),
            super::Card::new(super::Value::King, super::Suit::Club),
        ]);
        assert_eq!(super::Rank::StraightFlush(9), straight_flush.rank());

        let flush = super::Hand::new([
            super::Card::new(super::Value::Two, super::Suit::Club),
            super::Card::new(super::Value::Five, super::Suit::Club),
            super::Card::new(super::Value::Nine, super::Suit::Club),
            super::Card::new(super::Value::Jack, super::Suit::Club),
            super::Card::new(super::Value::King, super::Suit::Club),
        ]);
        assert_eq!(super::Rank::Flush(super::Value::King as u64), flush.rank());

        let four_of_a_kind = super::Hand::new([
            super::Card::new(super::Value::Two, super::Suit::Diamond),
            super::Card::new(super::Value::Two, super::Suit::Heart),
            super::Card::new(super::Value::Two, super::Suit::Club),
            super::Card::new(super::Value::Two, super::Suit::Spade),
            super::Card::new(super::Value::King, super::Suit::Club),
        ]);
        assert_eq!(
            super::Rank::FourOfAKind(super::Value::Two as u64),
            four_of_a_kind.rank()
        );

        let full_house = super::Hand::new([
            super::Card::new(super::Value::Two, super::Suit::Diamond),
            super::Card::new(super::Value::Two, super::Suit::Heart),
            super::Card::new(super::Value::Two, super::Suit::Club),
            super::Card::new(super::Value::King, super::Suit::Spade),
            super::Card::new(super::Value::King, super::Suit::Club),
        ]);
        assert_eq!(
            super::Rank::FullHouse(super::Value::Two as u64),
            full_house.rank()
        );

        let straight = super::Hand::new([
            super::Card::new(super::Value::Nine, super::Suit::Club),
            super::Card::new(super::Value::Ten, super::Suit::Club),
            super::Card::new(super::Value::Jack, super::Suit::Club),
            super::Card::new(super::Value::Queen, super::Suit::Club),
            super::Card::new(super::Value::King, super::Suit::Spade),
        ]);
        assert_eq!(super::Rank::Straight(9), straight.rank());

        let three_of_a_kind = super::Hand::new([
            super::Card::new(super::Value::Five, super::Suit::Diamond),
            super::Card::new(super::Value::Five, super::Suit::Heart),
            super::Card::new(super::Value::Five, super::Suit::Club),
            super::Card::new(super::Value::Seven, super::Suit::Spade),
            super::Card::new(super::Value::King, super::Suit::Club),
        ]);
        assert_eq!(
            super::Rank::ThreeOfAKind(
                ((super::Value::Five as u64) << 13)
                    | super::Value::Seven as u64
                    | super::Value::King as u64
            ),
            three_of_a_kind.rank()
        );

        let two_pair = super::Hand::new([
            super::Card::new(super::Value::Five, super::Suit::Diamond),
            super::Card::new(super::Value::Five, super::Suit::Heart),
            super::Card::new(super::Value::Seven, super::Suit::Club),
            super::Card::new(super::Value::Seven, super::Suit::Spade),
            super::Card::new(super::Value::King, super::Suit::Club),
        ]);
        assert_eq!(
            super::Rank::TwoPair(
                (super::Value::Seven as u64) << 26
                    | (super::Value::Five as u64) << 13
                    | super::Value::King as u64
            ),
            two_pair.rank()
        );

        let one_pair = super::Hand::new([
            super::Card::new(super::Value::Five, super::Suit::Diamond),
            super::Card::new(super::Value::Five, super::Suit::Heart),
            super::Card::new(super::Value::Seven, super::Suit::Club),
            super::Card::new(super::Value::Nine, super::Suit::Spade),
            super::Card::new(super::Value::King, super::Suit::Club),
        ]);
        assert_eq!(
            super::Rank::OnePair(
                (super::Value::Five as u64) << 13
                    | super::Value::Seven as u64
                    | super::Value::Nine as u64
                    | super::Value::King as u64
            ),
            one_pair.rank()
        );

        let high_card = super::Hand::new([
            super::Card::new(super::Value::Five, super::Suit::Diamond),
            super::Card::new(super::Value::Seven, super::Suit::Heart),
            super::Card::new(super::Value::Eight, super::Suit::Club),
            super::Card::new(super::Value::Jack, super::Suit::Spade),
            super::Card::new(super::Value::King, super::Suit::Club),
        ]);
        assert_eq!(
            super::Rank::HighCard(high_card.value_mask()),
            high_card.rank()
        );

        assert!(royal_flush.rank() > straight_flush.rank());
        assert!(
            straight_flush.rank()
                > super::Hand::new([
                    super::Card::new(super::Value::Three, super::Suit::Club),
                    super::Card::new(super::Value::Four, super::Suit::Club),
                    super::Card::new(super::Value::Five, super::Suit::Club),
                    super::Card::new(super::Value::Six, super::Suit::Club),
                    super::Card::new(super::Value::Seven, super::Suit::Club),
                ])
                .rank()
        );
        assert!(straight_flush.rank() > flush.rank());
    }
}
