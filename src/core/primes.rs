use std::collections::{BTreeMap, HashMap};

pub fn is_prime(p: usize) -> bool {
    for d in 2..p / 2 {
        if p % d == 0 {
            return false;
        }
    }

    true
}

#[derive(Clone)]
pub struct Primes {
    state: State,
    candidate: usize,
    sieve: HashMap<usize, (usize, usize)>,
    base_primes: Box<Option<Primes>>,
    p: usize,
    q: usize,
}

impl Default for Primes {
    fn default() -> Self {
        Self {
            state: State::Two,
            candidate: 7,
            sieve: HashMap::new(),
            base_primes: Box::new(None),
            p: 0,
            q: 0,
        }
    }
}

impl Iterator for Primes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.state {
            State::Two => {
                self.state = State::Three;
                Some(2)
            }
            State::Three => {
                self.state = State::Five;
                Some(3)
            }
            State::Five => {
                self.state = State::Seven;
                Some(5)
            }
            State::Seven => {
                self.state = State::More;
                Some(7)
            }
            State::More => {
                let mut internal = std::mem::replace(&mut self.base_primes, Box::new(None))
                    .unwrap_or_else(|| {
                        let mut primes = Self::default();
                        primes.next();
                        primes
                    });

                if self.q == 0 {
                    self.p = internal.next().expect("next prime can be generated");
                    self.q = self.p * self.p;
                }

                loop {
                    self.candidate += 2;
                    let step = if let Some(step) = self.sieve.remove(&self.candidate) {
                        step
                    } else if self.candidate < self.q {
                        self.base_primes = Box::new(Some(internal));
                        self.state = State::More;
                        return Some(self.candidate);
                    } else {
                        assert_eq!(self.candidate, self.q);
                        let step = (self.q + 2 * self.p, 2 * self.p);
                        self.p = internal.next().expect("next prime can be generated");
                        self.q = self.p * self.p;
                        step
                    };

                    let mut b = step.0;
                    for m in (step.0..).step_by(step.1) {
                        if !self.sieve.contains_key(&m) {
                            b = m;
                            break;
                        }
                    }

                    self.sieve.insert(b, (b + step.1, step.1));
                }
            }
        }
    }
}

#[derive(Clone)]
enum State {
    Two,
    Three,
    Five,
    Seven,
    More,
}

pub type PrimeFactors = BTreeMap<usize, usize>;

pub fn factors(mut n: usize) -> PrimeFactors {
    let mut prime_factors = BTreeMap::new();

    let mut count = 0;
    while n & 1 == 0 {
        n >>= 1;
        count += 1;
    }

    if count > 0 {
        prime_factors.insert(2, count);
    }

    let mut factor = 3;
    while factor * factor <= n {
        count = 0;
        while n % factor == 0 {
            n /= factor;
            count += 1;
        }
        if count > 0 {
            prime_factors.insert(factor, count);
        }

        factor += 2;
    }

    if n > 1 {
        prime_factors.insert(n, 1);
    }

    prime_factors
}

pub trait Divisors {
    fn divisors(&self) -> Vec<usize>;
}

fn divisors(value: usize, factors: &PrimeFactors, factor: &mut Vec<usize>, divs: &mut Vec<usize>) {
    match factor.pop() {
        None => {
            divs.push(value);
        }
        Some(f) => {
            let power = factors.get(&f).cloned().unwrap_or_default();
            for p in 0..=power {
                divisors(
                    value * (f.pow(p as u32)),
                    factors,
                    &mut factor.clone(),
                    divs,
                );
            }
        }
    }
}

impl Divisors for PrimeFactors {
    fn divisors(&self) -> Vec<usize> {
        let mut factors = self.keys().cloned().collect::<Vec<_>>();
        let mut divs = vec![];
        divisors(1, self, &mut factors, &mut divs);
        divs
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_primes() {
        let primes = super::Primes::default();
        let mut iter = primes.into_iter();
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(3), iter.next());
        assert_eq!(Some(5), iter.next());
        assert_eq!(Some(7), iter.next());
        assert_eq!(Some(11), iter.next());
        assert_eq!(Some(13), iter.next());
        assert_eq!(Some(17), iter.next());
        assert_eq!(Some(19), iter.next());
        assert_eq!(Some(23), iter.next());
        assert_eq!(Some(29), iter.next());
        assert_eq!(Some(31), iter.next());
    }

    #[test]
    fn test_factors() {
        let factors = super::factors(13195);
        assert_eq!(4, factors.len());
        assert!(factors.contains_key(&5));
        assert!(factors.contains_key(&7));
        assert!(factors.contains_key(&13));
        assert!(factors.contains_key(&29));
    }

    #[test]
    fn test_divisors() {
        use crate::core::primes::Divisors;

        let divisors = super::factors(220).divisors();
        assert_eq!(
            divisors.as_slice(),
            [1, 2, 4, 5, 10, 20, 11, 22, 44, 55, 110, 220]
        );
    }
}
