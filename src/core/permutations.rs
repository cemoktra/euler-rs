enum State {
    SingleDigit(u8),
    MultipleDigits {
        digits: Vec<u8>,
        inner: Box<Permutations>,
    },
}

pub struct Permutations {
    state: State,
    current: usize,
}

impl Permutations {
    pub fn new(mut possible_digits: Vec<u8>) -> Self {
        possible_digits.sort();

        if possible_digits.len() > 1 {
            Self {
                state: State::MultipleDigits {
                    inner: Box::new(Permutations::new(possible_digits.as_slice()[1..].to_vec())),
                    digits: possible_digits,
                },
                current: 0,
            }
        } else {
            Self {
                state: State::SingleDigit(possible_digits[0]),
                current: 0,
            }
        }
    }
}

impl Iterator for Permutations {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            State::SingleDigit(digit) => {
                if self.current > 0 {
                    None
                } else {
                    self.current += 1;
                    Some(vec![*digit])
                }
            }
            State::MultipleDigits {
                digits,
                ref mut inner,
            } => match inner.next() {
                None => {
                    if self.current < digits.len() - 1 {
                        self.current += 1;
                        let mut inner_digits = digits.clone();
                        inner_digits.remove(self.current);

                        self.state = State::MultipleDigits {
                            inner: Box::new(Permutations::new(inner_digits)),
                            digits: digits.clone(),
                        };

                        return self.next();
                    }
                    None
                }
                Some(mut inner_vec) => {
                    inner_vec.insert(0, digits[self.current]);
                    Some(inner_vec)
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let mut permutations = super::Permutations::new(vec![1, 2, 3, 0]);
        assert_eq!(Some(vec![0, 1, 2, 3]), permutations.next());
        assert_eq!(Some(vec![0, 1, 3, 2]), permutations.next());
        assert_eq!(Some(vec![0, 2, 1, 3]), permutations.next());
        assert_eq!(Some(vec![0, 2, 3, 1]), permutations.next());
        assert_eq!(Some(vec![0, 3, 1, 2]), permutations.next());
        assert_eq!(Some(vec![0, 3, 2, 1]), permutations.next());

        assert_eq!(Some(vec![1, 0, 2, 3]), permutations.next());
        assert_eq!(Some(vec![1, 0, 3, 2]), permutations.next());
        assert_eq!(Some(vec![1, 2, 0, 3]), permutations.next());
        assert_eq!(Some(vec![1, 2, 3, 0]), permutations.next());
        assert_eq!(Some(vec![1, 3, 0, 2]), permutations.next());
        assert_eq!(Some(vec![1, 3, 2, 0]), permutations.next());

        assert_eq!(Some(vec![2, 0, 1, 3]), permutations.next());
        assert_eq!(Some(vec![2, 0, 3, 1]), permutations.next());
        assert_eq!(Some(vec![2, 1, 0, 3]), permutations.next());
        assert_eq!(Some(vec![2, 1, 3, 0]), permutations.next());
        assert_eq!(Some(vec![2, 3, 0, 1]), permutations.next());
        assert_eq!(Some(vec![2, 3, 1, 0]), permutations.next());

        assert_eq!(Some(vec![3, 0, 1, 2]), permutations.next());
        assert_eq!(Some(vec![3, 0, 2, 1]), permutations.next());
        assert_eq!(Some(vec![3, 1, 0, 2]), permutations.next());
        assert_eq!(Some(vec![3, 1, 2, 0]), permutations.next());
        assert_eq!(Some(vec![3, 2, 0, 1]), permutations.next());
        assert_eq!(Some(vec![3, 2, 1, 0]), permutations.next());

        assert_eq!(None, permutations.next());
    }
}
