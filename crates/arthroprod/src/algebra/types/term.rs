use itertools::Itertools;

use std::{cmp, fmt, ops};

use crate::algebra::{
    ar_product, Alpha, ArInput, ArIterator, ArOutput, Form, Grade, Magnitude, MultiVector, Sign,
    Xi, AR,
};

/// A Term represents a real scalar magnitude along with a paired [`Alpha`] giving the
/// proper Space-Time [`Form`] in accordence with the principle of Absolute Relativity.
#[derive(Hash, Eq, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Term {
    magnitude: Magnitude,
    alpha: Alpha,
    xi: Xi,
}

impl Term {
    /// Construct a new Term. The underlying symbolic value will be constructed
    /// from the Form of alpha if None is provided.
    pub fn new(val: Option<&str>, alpha: Alpha) -> Term {
        let xi = if let Some(v) = val {
            Xi::new(v)
        } else {
            Xi::new(&format!("{}", alpha.form()))
        };

        Term {
            magnitude: 1u8.into(),
            alpha,
            xi,
        }
    }

    /// Construct a Term with compoud Xi values as opposed to raw symbols
    pub fn from_xis_and_alpha(xis: Vec<&str>, alpha: Alpha) -> Term {
        Term {
            magnitude: 1u8.into(),
            alpha,
            xi: Xi::merge(&xis.iter().map(|s| Xi::new(s)).collect_vec()),
        }
    }

    /// Extract a copy of the Space-Time [`Form`] of this term
    pub fn form(&self) -> Form {
        self.alpha.form()
    }

    /// Extract a copy of the Space-Time [`Grade`] of this term
    pub fn grade(&self) -> Grade {
        self.alpha.grade()
    }

    /// Extract the [`Sign`] of this Term
    pub fn sign(&self) -> Sign {
        self.alpha.sign()
    }

    /// Extract a copy of the [`Alpha`] of this Term
    pub fn alpha(&self) -> Alpha {
        self.alpha
    }

    /// Extract the unsigned [`Magnitude`] of this Term
    pub fn magnitude(&self) -> Magnitude {
        self.magnitude
    }

    pub(crate) fn mag_str(&self) -> String {
        if self.magnitude != 1u8 {
            self.magnitude.to_string()
        } else {
            "".to_string()
        }
    }

    /// Override the Alpha value of this Term
    pub fn set_alpha(&mut self, a: Alpha) {
        self.alpha = a;
    }

    /// Add a single partial derivative and resort
    pub fn add_partial(&mut self, wrt: &Alpha) {
        self.xi.add_partial(&wrt.form())
    }

    /// Replace the current set of partial derivatives
    pub fn set_partials(&mut self, partials: Vec<Form>) {
        self.xi.set_partials(partials)
    }

    /// Generate a string representation of the underlying Xi values for this term
    pub fn xi_str(&self) -> String {
        format!("{}", self.xi)
    }

    /// Attempt to add two Terms. This will only succeed if their summation_key
    /// of both Terms is the same. We use this as a method rather than implimenting
    /// ops::Add for Terms as we are not guaranteed to be able to return a result.
    pub fn try_add(&self, other: &Term) -> Option<Term> {
        fn sub_mag(a: &Term, b: &Term) -> Term {
            // For subtraction we need to make sure that magnitude stays positive
            // so we flip the sign of the alpha if needed and make use of the fact
            // that A - B == -(B - A)
            let mut t = a.clone();
            if t.magnitude > b.magnitude {
                t.magnitude -= b.magnitude;
            } else {
                t.magnitude = b.magnitude - t.magnitude;
                t.alpha = -t.alpha;
            }

            t
        }

        if self.summation_key() == other.summation_key() {
            Some(match (self.sign(), other.sign()) {
                (Sign::Pos, Sign::Pos) | (Sign::Neg, Sign::Neg) => {
                    let mut t = self.clone();
                    t.magnitude += other.magnitude;
                    t
                }
                (Sign::Pos, Sign::Neg) => sub_mag(self, other), // sub other from self
                (Sign::Neg, Sign::Pos) => sub_mag(other, self), // sub self from other
            })
        } else {
            None
        }
    }

    /// Form the product of this term and another under the full product of the algebra
    pub fn form_product_with(&self, other: &Term) -> Term {
        Term {
            magnitude: self.magnitude * other.magnitude,
            alpha: ar_product(&self.alpha, &other.alpha),
            xi: Xi::merge(&[self.xi.clone(), other.xi.clone()]),
        }
    }

    /// The elements of a Term that need to match for us to be able to sum them
    pub fn summation_key(&self) -> (Form, String) {
        (self.form(), self.xi_str())
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m_str = if self.magnitude != 1u8 {
            format!("({})", self.magnitude)
        } else {
            String::new()
        };

        write!(f, "{}{}({})", self.alpha, m_str, self.xi_str())
    }
}

impl AsRef<Form> for Term {
    fn as_ref(&self) -> &Form {
        self.alpha.as_ref()
    }
}

impl AsRef<Alpha> for Term {
    fn as_ref(&self) -> &Alpha {
        &self.alpha
    }
}

impl ArInput for Term {
    fn as_terms(&self) -> Vec<Term> {
        vec![self.clone()]
    }

    fn ar_iter(&self) -> ArIterator {
        ArIterator::from_ref_term(self)
    }

    fn norm(&self) -> Magnitude {
        self.magnitude
    }
}

impl ArOutput for Term {
    fn from_terms(terms: Vec<Term>) -> Self {
        if terms.len() != 1 {
            panic!("Can only construct an Term from a single term")
        };

        terms[0].clone()
    }
}

impl AR for Term {
    fn inverse(&self) -> Self {
        Term {
            magnitude: 1usize / self.magnitude,
            alpha: self.alpha.inverse(),
            xi: self.xi.inverse(),
        }
    }
}

// Supported arithmetic operations:
//   - negation
//   - addition:        term
//   - subtraction:     term
//   - multiplication:  uX, iX, magnitude, alpha, term
//   - division:        uX, iX, magnitude, alpha, term
//   - mul-assign:      uX, iX, magnitude, alpha, term
//   - div-assign:      uX, iX, magnitude, alpha, term

macro_rules! __arith_impl {
    (@inner $t:ty: $op:ident $func:ident; $conv:expr) => {
        impl ops::$op<$t> for Term {
            type Output = Self;

            fn $func(self, rhs: $t) -> Self::Output {
                let (sgn, r): (Sign, Magnitude) = $conv(rhs);
                let l = match sgn {
                    Sign::Pos => self,
                    Sign::Neg => -self,
                };

                ops::$op::$func(l, r)
            }
        }

        impl ops::$op<Term> for $t {
            type Output = Term;

            fn $func(self, rhs: Term) -> Self::Output {
                let (sgn, l): (Sign, Magnitude) = $conv(self);
                let r = match sgn {
                    Sign::Pos => rhs,
                    Sign::Neg => -rhs,
                };

                ops::$op::$func(l, r)
            }
        }
    };

    ($t:ty; $conv:expr) => {
        __arith_impl!(@inner $t: Mul mul; $conv);
        __arith_impl!(@inner $t: Div div; $conv);

        impl ops::MulAssign<$t> for Term {
            fn mul_assign(&mut self, other: $t) {
                let (sgn, r): (Sign, Magnitude) = $conv(other);
                if let Sign::Neg = sgn {
                    self.alpha = self.alpha.inverse();
                }
                *self *= r;
            }
        }

        impl ops::DivAssign<$t> for Term {
            fn div_assign(&mut self, other: $t) {
                let (sgn, r): (Sign, Magnitude) = $conv(other);
                if let Sign::Neg = sgn {
                    self.alpha = self.alpha.inverse();
                }
                *self /= r;
            }
        }
    };
}

fn mag(u: usize) -> Magnitude {
    Magnitude::from(u)
}

macro_rules! __sgn_mag {
    ($u:expr) => {
        if $u < 0 {
            (Sign::Neg, mag(-$u as usize))
        } else {
            (Sign::Pos, mag($u as usize))
        }
    };
}

__arith_impl!(u8;    |u| (Sign::Pos, mag(u as usize)));
__arith_impl!(u16;   |u| (Sign::Pos, mag(u as usize)));
__arith_impl!(u32;   |u| (Sign::Pos, mag(u as usize)));
__arith_impl!(usize; |u| (Sign::Pos, mag(u as usize)));
__arith_impl!(i8;    |u: i8|    __sgn_mag!(u));
__arith_impl!(i16;   |u: i16|   __sgn_mag!(u));
__arith_impl!(i32;   |u: i32|   __sgn_mag!(u));
__arith_impl!(isize; |u: isize| __sgn_mag!(u));

impl ops::Neg for Term {
    type Output = Term;

    fn neg(self) -> Self::Output {
        let mut t = self;
        t.alpha = -t.alpha;

        t
    }
}

impl ops::Add for Term {
    type Output = MultiVector;

    fn add(self, rhs: Term) -> Self::Output {
        MultiVector::from_terms(vec![self, rhs])
    }
}

impl ops::Sub for Term {
    type Output = MultiVector;

    fn sub(self, rhs: Term) -> Self::Output {
        MultiVector::from_terms(vec![self, -rhs])
    }
}

impl ops::Mul<Magnitude> for Term {
    type Output = Self;

    fn mul(self, rhs: Magnitude) -> Self::Output {
        let mut t = self;
        t.magnitude *= rhs;

        t
    }
}

impl ops::Mul<Term> for Magnitude {
    type Output = Term;

    fn mul(self, rhs: Term) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Alpha> for Term {
    type Output = Self;

    fn mul(self, rhs: Alpha) -> Self::Output {
        let mut t = self;
        t.alpha *= rhs;

        t
    }
}

impl ops::Mul<Term> for Alpha {
    type Output = Term;

    fn mul(self, rhs: Term) -> Self::Output {
        let mut t = rhs;
        t.alpha = self * t.alpha;

        t
    }
}

impl ops::Mul<Term> for Term {
    type Output = Self;

    fn mul(self, rhs: Term) -> Self::Output {
        self.form_product_with(&rhs)
    }
}

impl ops::Div<Magnitude> for Term {
    type Output = Self;

    fn div(self, rhs: Magnitude) -> Self::Output {
        let mut t = self;
        t.magnitude /= rhs;

        t
    }
}

impl ops::Div<Term> for Magnitude {
    type Output = Term;

    fn div(self, rhs: Term) -> Self::Output {
        let mut t = rhs;
        t.magnitude = self / t.magnitude;
        t.alpha = t.alpha.inverse();

        t
    }
}

impl ops::Div<Alpha> for Term {
    type Output = Self;

    fn div(self, rhs: Alpha) -> Self::Output {
        let mut t = self;
        t.alpha /= rhs;

        t
    }
}

impl ops::Div<Term> for Alpha {
    type Output = Term;

    fn div(self, rhs: Term) -> Self::Output {
        let mut t = rhs;
        t.magnitude = 1usize / t.magnitude;
        t.alpha = self / t.alpha;

        t
    }
}

impl ops::Div<Term> for Term {
    type Output = Self;

    fn div(self, rhs: Term) -> Self::Output {
        self.form_product_with(&rhs.inverse())
    }
}

impl ops::MulAssign<Magnitude> for Term {
    fn mul_assign(&mut self, other: Magnitude) {
        self.magnitude *= other;
    }
}

impl ops::DivAssign<Magnitude> for Term {
    fn div_assign(&mut self, other: Magnitude) {
        self.magnitude /= other;
    }
}

impl ops::MulAssign<Alpha> for Term {
    fn mul_assign(&mut self, other: Alpha) {
        self.alpha *= other;
    }
}

impl ops::DivAssign<Alpha> for Term {
    fn div_assign(&mut self, other: Alpha) {
        self.alpha /= other;
    }
}

impl ops::MulAssign<Term> for Term {
    fn mul_assign(&mut self, other: Term) {
        self.magnitude *= other.magnitude;
        self.alpha *= other.alpha;
    }
}

impl ops::DivAssign<Term> for Term {
    fn div_assign(&mut self, other: Term) {
        self.magnitude /= other.magnitude;
        self.alpha /= other.alpha;
    }
}

impl cmp::Ord for Term {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.form()
            .cmp(&other.form())
            .then(self.xi.cmp(&other.xi))
            .then(self.sign().cmp(&other.sign()))
            .then(self.magnitude.cmp(&other.magnitude))
    }
}

impl cmp::PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_cases! {
        summation_key;
        args: (t: Term, u: Term, expected: bool);

        case: pos_match => (term!("foo", 1 2 3), term!("foo", 1 2 3), true);
        case: neg_match => (term!("foo", 1), -term!("foo", 1), true);
        case: other_alpha => (term!("foo", 1), term!("foo", 2), false);
        case: other_xi => (term!("foo", 0 2), term!("bar", 0 2), false);
        case: other_alpha_no_xi => (term!(1), term!(2), false);
        case: other_alpha_multiple => (term!(1), 2 * term!(2), false);
        case: other_alpha_multiple_neg => (term!(1), -2 * term!(2), false);

        body: {
            assert_eq!(t.summation_key() == u.summation_key(), expected);
        }
    }

    test_cases! {
        try_add;
        args: (t: Term, u: Term, expected: Option<Term>);

        case: cant_sum => (term!(1), -2 * term!(2), None);
        case: pos_pos => (term!(1), term!(1), Some(2 * term!(1)));
        case: pos_neg => (term!(1), -2 * term!(1), Some(-term!(1)));
        case: neg_pos => (term!(1) * -2, term!(1), Some(-term!(1)));
        case: neg_neg => (term!(1) * -2, -2 * term!(1), Some(-4 * term!(1)));

        body: {
            assert_eq!(t.try_add(&u), expected);
        }
    }

    test_cases! {
        form_product_no_inversion;
        args: (left: Term, right: Term, expected: Term);

        case: different_xi => (term!("a", 2 3), term!("b", 1 2 3), -term!(["a", "b"], 1));
        case: same_xi => (term!("a", 2 3), term!("a", 2 3), -term!(["a", "a"], ));

        body: {
            assert_eq!(left.form_product_with(&right), expected)
        }
    }
}
