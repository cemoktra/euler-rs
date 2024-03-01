use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub struct UBig<const N: usize> {
    num: [usize; N],
}

impl<const N: usize> UBig<N> {
    pub fn zero() -> Self {
        Self { num: [0; N] }
    }

    pub fn digits(&self) -> impl DoubleEndedIterator<Item = &usize> {
        self.num.iter()
    }
}

impl<const N: usize> Display for UBig<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for d in self.num.iter().skip_while(|n| **n == 0) {
            write!(f, "{d}")?;
        }
        Ok(())
    }
}

impl<const N: usize> From<usize> for UBig<N> {
    fn from(mut value: usize) -> Self {
        let mut num = [0; N];
        num.iter_mut().for_each(|d| {
            *d = value % 10;
            value /= 10;
        });

        num.reverse();
        Self { num }
    }
}

impl<const N: usize> From<&[u8]> for UBig<N> {
    fn from(value: &[u8]) -> Self {
        let mut num = [0; N];
        num.iter_mut()
            .rev()
            .zip(value.iter().rev())
            .for_each(|(n, v)| *n = (*v).into());
        Self { num }
    }
}

impl<const N: usize> PartialEq for UBig<N> {
    fn eq(&self, other: &Self) -> bool {
        self.num.eq(&other.num)
    }
}

impl<const N: usize> std::ops::AddAssign for UBig<N> {
    fn add_assign(&mut self, rhs: Self) {
        let mut carry = 0;
        self.num
            .iter_mut()
            .rev()
            .zip(rhs.num.iter().rev())
            .for_each(|(a, b)| {
                let x = *a + *b + carry;
                let v = x % 10;
                carry = x / 10;
                *a = v;
            });
    }
}

impl<const N: usize> std::ops::Add for UBig<N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const N: usize> std::ops::MulAssign for UBig<N> {
    fn mul_assign(&mut self, rhs: Self) {
        let mut num = [0; N];
        let mut first = true;
        let mut carry;

        for rhs_digit in rhs.digits().skip_while(|n| **n == 0) {
            if !first {
                num.rotate_left(1);
            }
            first = false;
            carry = 0;
            for (d, r) in self.digits().rev().zip(num.iter_mut().rev()) {
                let p = rhs_digit * d;
                let v = p % 10;

                *r += v + carry;
                carry = p / 10;
            }
        }

        carry = 0;
        num.iter_mut().rev().for_each(|v| {
            let r = *v % 10 + carry;
            carry = *v / 10;
            *v = r;
        });

        self.num = num;
    }
}

impl<const N: usize> std::ops::Mul for UBig<N> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_add() {
        let x = super::UBig::<20>::zero();
        let y: super::UBig<20> = [1, 2, 3, 4, 5].as_slice().into();
        let z = x + y;

        assert_eq!(y, z);
    }

    #[test]
    fn test_mul() {
        let x: super::UBig<20> = [1, 2, 3].as_slice().into();
        let y: super::UBig<20> = [1, 2, 3, 4, 5].as_slice().into();
        let expected: super::UBig<20> = [1, 5, 1, 8, 4, 3, 5].as_slice().into();
        let z = x * y;

        assert_eq!(z, expected);
    }
}
