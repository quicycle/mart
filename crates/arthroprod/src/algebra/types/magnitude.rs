//! Magnitude is a simple rational number type that is intended for tracking term magnitudes within
//! algebraic calculations. It is not intended for use within numeric computation or simulation.
//! The numerator and denominator are always stored in lowest terms and opemagnitudens will panic
//! if the denominator is set to zero.
//! NOTE: division of Magnitudes is defined in standard (lhs / rhs) not (lhs \ rhs) as with division
//!       for AR. This is handled when working with Xi terms but should be taken into account if
//!       you want to manipulate raw Magnitude values.

// TODO: need to handle roots in order to be able to compute MultiVector norms...

use std::{cmp, convert, fmt, ops};

/// A Magnitude is a strictly positive rational number. Sign (as it pertains to directed elements)
/// is stored in the Alpha value describine the element.
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Magnitude {
    numerator: usize,
    denominator: usize,
}

impl Magnitude {
    /// Construct a new Magnitude ratio and cancel it to be in lowest terms
    pub fn new(numerator: usize, denominator: usize) -> Magnitude {
        let mut r = Magnitude::new_unchecked(numerator, denominator);
        r.reduce();
        r
    }

    /// Check if this magnitude is non-zero
    pub fn is_non_zero(&self) -> bool {
        self.numerator != 0
    }

    fn new_unchecked(numerator: usize, denominator: usize) -> Magnitude {
        Magnitude {
            numerator,
            denominator,
        }
    }

    fn reduce(&mut self) {
        if self.denominator == 0 {
            panic!("magnitude denominator is 0")
        }
        if self.numerator == 0 {
            self.denominator = 1;
            return;
        }
        if self.numerator == self.denominator {
            self.numerator = 1;
            self.denominator = 1;
            return;
        }

        let g = gcd(self.numerator, self.denominator);
        self.numerator /= g;
        self.denominator /= g;
    }
}

fn gcd(n: usize, m: usize) -> usize {
    let mut a = n;
    let mut b = m;

    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    a
}

impl fmt::Display for Magnitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.denominator {
            1 => write!(f, "{}", self.numerator),
            _ => write!(f, "{}/{}", self.numerator, self.denominator),
        }
    }
}

// Supported arithmetic operations:
//   - negation
//   - addition:        uX, magnitude
//   - subtraction:     uX, magnitude
//   - multiplication:  uX, magnitude
//   - division:        uX, magnitude
//   - add-assign:      uX, magnitude
//   - sub-assign:      uX, magnitude
//   - mul-assign:      uX, magnitude
//   - div-assign:      uX, magnitude

macro_rules! __uimpl {
    ($t:ty) => {
        impl cmp::PartialEq<$t> for Magnitude {
            fn eq(&self, other: &$t) -> bool {
                self.denominator == 1usize && self.numerator == (*other as usize)
            }
        }

        impl cmp::PartialEq<Magnitude> for $t {
            fn eq(&self, other: &Magnitude) -> bool {
                other == self
            }
        }

        impl convert::From<$t> for Magnitude {
            fn from(num: $t) -> Self {
                Magnitude::new_unchecked(num as usize, 1)
            }
        }

        // e.g. let rat: Magnitude = (5, 3).into();
        impl convert::From<($t, $t)> for Magnitude {
            fn from(pair: ($t, $t)) -> Self {
                Magnitude::new(pair.0 as usize, pair.1 as usize)
            }
        }

        impl convert::TryFrom<Vec<$t>> for Magnitude {
            type Error = String;
            fn try_from(v: Vec<$t>) -> Result<Self, Self::Error> {
                match v.len() {
                    1 => Ok(Magnitude::new(v[0] as usize, 1)),
                    2 => Ok(Magnitude::new(v[0] as usize, v[1] as usize)),
                    _ => Err("can only create a Magnitude from a vector of 1 or 2 elements".into()),
                }
            }
        }

        impl convert::From<Magnitude> for ($t, $t) {
            fn from(m: Magnitude) -> ($t, $t) {
                (m.numerator as $t, m.denominator as $t)
            }
        }

        impl ops::Add<$t> for Magnitude {
            type Output = Self;
            fn add(self, rhs: $t) -> Self::Output {
                Magnitude::new(
                    self.numerator + (rhs as usize) * self.denominator,
                    self.denominator,
                )
            }
        }

        impl ops::Add<Magnitude> for $t {
            type Output = Magnitude;
            fn add(self, rhs: Magnitude) -> Self::Output {
                Magnitude::new(
                    rhs.numerator + (self as usize) * rhs.denominator,
                    rhs.denominator,
                )
            }
        }

        impl ops::Sub<$t> for Magnitude {
            type Output = Self;
            fn sub(self, rhs: $t) -> Self::Output {
                Magnitude::new(
                    self.numerator - (rhs as usize) * self.denominator,
                    self.denominator,
                )
            }
        }

        impl ops::Sub<Magnitude> for $t {
            type Output = Magnitude;
            fn sub(self, rhs: Magnitude) -> Self::Output {
                Magnitude::new(
                    rhs.numerator - (self as usize) * rhs.denominator,
                    rhs.denominator,
                )
            }
        }

        impl ops::Mul<$t> for Magnitude {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Magnitude::new(self.numerator * (rhs as usize), self.denominator)
            }
        }

        impl ops::Mul<Magnitude> for $t {
            type Output = Magnitude;
            fn mul(self, rhs: Magnitude) -> Self::Output {
                Magnitude::new((self as usize) * rhs.numerator, rhs.denominator)
            }
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        impl ops::Div<$t> for Magnitude {
            type Output = Self;
            fn div(self, rhs: $t) -> Self::Output {
                Magnitude::new(self.numerator, self.denominator * (rhs as usize))
            }
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        impl ops::Div<Magnitude> for $t {
            type Output = Magnitude;
            fn div(self, rhs: Magnitude) -> Self::Output {
                Magnitude::new((self as usize) * rhs.denominator, rhs.numerator)
            }
        }

        impl ops::AddAssign<$t> for Magnitude {
            fn add_assign(&mut self, other: $t) {
                *self += Magnitude::from(other as usize)
            }
        }

        impl ops::SubAssign<$t> for Magnitude {
            fn sub_assign(&mut self, other: $t) {
                *self -= Magnitude::from(other as usize)
            }
        }

        impl ops::MulAssign<$t> for Magnitude {
            fn mul_assign(&mut self, other: $t) {
                *self *= Magnitude::from(other as usize);
            }
        }

        impl ops::DivAssign<$t> for Magnitude {
            fn div_assign(&mut self, other: $t) {
                *self /= Magnitude::from(other as usize);
            }
        }
    };
}

__uimpl!(u8);
__uimpl!(u16);
__uimpl!(u32);
__uimpl!(usize);

impl cmp::Ord for Magnitude {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // NOTE: this is in danger of overflowing but for our use case we will typically be fine.
        (self.numerator * other.denominator).cmp(&(self.denominator * other.numerator))
    }
}

impl cmp::PartialOrd for Magnitude {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ops::Add for Magnitude {
    type Output = Self;

    fn add(self, rhs: Magnitude) -> Self::Output {
        let num = (self.numerator * rhs.denominator) + (rhs.numerator * self.denominator);
        let den = self.denominator * rhs.denominator;

        Magnitude::new(num, den)
    }
}

impl ops::Sub for Magnitude {
    type Output = Self;

    fn sub(self, rhs: Magnitude) -> Self::Output {
        let num = (self.numerator * rhs.denominator) - (rhs.numerator * self.denominator);
        let den = self.denominator * rhs.denominator;

        Magnitude::new(num, den)
    }
}

impl ops::Mul for Magnitude {
    type Output = Self;

    fn mul(self, rhs: Magnitude) -> Self::Output {
        Magnitude::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl ops::Div for Magnitude {
    type Output = Self;

    fn div(self, rhs: Magnitude) -> Self::Output {
        Magnitude::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

impl ops::AddAssign for Magnitude {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl ops::SubAssign for Magnitude {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl ops::MulAssign for Magnitude {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl ops::DivAssign for Magnitude {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mag(a: usize, b: usize) -> Magnitude {
        Magnitude::new(a, b)
    }

    #[test]
    fn equality_works() {
        assert_eq!(mag(2, 4), mag(1, 2));
        assert_eq!(mag(4, 2), 2u32);
        assert_eq!(5u32, mag(15, 3));
    }

    test_cases! {
        comparison;
        args: (left: Magnitude, right: Magnitude, ord: cmp::Ordering);

        case: identical => (mag(1, 2), mag(1, 2), cmp::Ordering::Equal);
        case: cancel_equivalent => (mag(1, 2), mag(5, 10), cmp::Ordering::Equal);
        case: less => (mag(1, 3), mag(2, 5), cmp::Ordering::Less);
        case: greater => (mag(3, 4), mag(5, 9), cmp::Ordering::Greater);

        body: {
            assert_eq!(left.cmp(&right), ord);
        }
    }

    test_cases! {
        new_in_lowest_terms;
        args: (a: usize, b: usize, num: usize, den: usize);

        case: already_simplified => (1, 2, 1, 2);
        case: half => (2, 4, 1, 2);
        case: third => (3, 9, 1, 3);
        case: five => (25, 5, 5, 1);

        body: {
            let m = mag(a, b);
            assert_eq!(m.numerator, num);
            assert_eq!(m.denominator, den);
        }
    }

    test_cases! {
        addition;
        args: (a: Magnitude, b: Magnitude, expected: Magnitude);

        case: denominator_shared => (mag(1, 3), mag(1, 3), mag(2, 3));
        case: denominator_multiple => (mag(1, 2), mag(3, 4), mag(5, 4));
        case: denominator_uncommon => (mag(3, 5), mag(4, 3), mag(29, 15));

        body: {
            assert_eq!(a + b, expected);
        }
    }

    test_cases! {
        addition_u_impl;
        args: (a: Magnitude, b: usize, expected: Magnitude);

        case: integral => (mag(3, 3), 3, mag(4, 1));
        case: fractional => (mag(1, 2), 1, mag(3, 2));

        body: {
            assert_eq!(a + b, expected);
            assert_eq!(b + a, expected);
        }
    }

    test_cases! {
        subtraction;
        args: (a: Magnitude, b: Magnitude, expected: Magnitude);

        case: denominator_shared => (mag(1, 3), mag(1, 3), mag(0, 1));
        case: denominator_multiple => (mag(3, 4), mag(1, 2), mag(1, 4));
        case: denominator_uncommon => (mag(4, 3), mag(3, 5), mag(11, 15));

        body: {
            assert_eq!(a - b, expected);
        }
    }

    test_cases! {
        subtraction_u_impl;
        args: (a: Magnitude, b: usize, expected: Magnitude);

        case: integral => (mag(3, 3), 1, mag(0, 1));
        case: fractional => (mag(3, 2), 1, mag(1, 2));

        body: {
            assert_eq!(a - b, expected);
        }
    }

    test_cases! {
        multiplication;
        args: (a: Magnitude, b: Magnitude, expected: Magnitude);

        case: denominator_shared => (mag(1, 2), mag(1, 2), mag(1, 4));
        case: denominator_multiple => (mag(3, 4), mag(1, 2), mag(3, 8));
        case: denominator_uncommon => (mag(2, 3), mag(1, 2), mag(1, 3));

        body: {
            assert_eq!(a * b, expected);
        }
    }

    test_cases! {
        multiplication_u_impl;
        args: (a: Magnitude, b: usize, expected: Magnitude);

        case: to_integral => (mag(1, 2), 2, mag(1, 1));
        case: to_integral_cancel => (mag(3, 5), 5, mag(3, 1));
        case: to_fractional => (mag(2, 5), 4, mag(8, 5));

        body: {
            assert_eq!(a * b, expected);
            assert_eq!(b * a, expected);
        }
    }

    test_cases! {
        division;
        args: (a: Magnitude, b: Magnitude, expected: Magnitude);

        case: denominator_shared => (mag(1, 2), mag(1, 2), mag(1, 1));
        case: denominator_multiple => (mag(1, 2), mag(1, 4), mag(2, 1));
        case: denominator_uncommon => (mag(3, 5), mag(4, 3), mag(9, 20));

        body: {
            assert_eq!(a / b, expected);
        }
    }

    test_cases! {
        division_u_impl;
        args: (a: Magnitude, b: usize, expected: Magnitude);

        case: integral => (mag(4, 1), 2, mag(2, 1));
        case: to_integral_cancel => (mag(9, 3), 3, mag(1, 1));
        case: to_fractional => (mag(2, 5), 4, mag(1, 10));

        body: {
            assert_eq!(a / b, expected);
        }
    }
}
