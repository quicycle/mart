use std::{collections::HashSet, fmt, ops};

use itertools::Itertools;

use crate::algebra::{
    div_by, full, Alpha, ArInput, ArIterator, ArOutput, Form, Magnitude, Term, AR,
};

/// A MultiVector is an ordered collection of a Terms representing a particular
/// composite quantity within the Algebra. In its simplest form, a MultiVector is
/// a simple linear sum of Alphas, though it is possible for there to be significantly
/// more structure.
///
/// In practice, almost all  computations are done using MultiVectors as their
/// primary data structure so there are a number of methods designed for aiding in
/// simplifying such computations.
#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct MultiVector {
    terms: Vec<Term>,
}

impl ArInput for MultiVector {
    fn as_terms(&self) -> Vec<Term> {
        self.terms.clone()
    }

    fn ar_iter(&self) -> ArIterator {
        ArIterator::FromSlice(self.iter())
    }
}

impl ArOutput for MultiVector {
    fn from_terms(mut terms: Vec<Term>) -> Self {
        terms.sort_unstable();
        MultiVector { terms }
    }
}

impl AR for MultiVector {}

impl MultiVector {
    /// Construct a new, empty MultiVector
    pub fn new() -> MultiVector {
        MultiVector { terms: vec![] }
    }

    pub fn from_ar<T: ArInput>(ar: T) -> Self {
        let mut terms = ar.as_terms();
        terms.sort_unstable();

        MultiVector { terms }
    }

    /// Returns an iterator over references to the terms contained in this MultiVector
    pub fn iter(&self) -> std::slice::Iter<Term> {
        self.terms.iter()
    }

    /// Push a new term into this MultiVector and reorder the terms if needed.
    pub fn push(&mut self, term: Term) {
        self.terms.push(term);
        self.terms.sort_unstable();
    }

    pub fn forms(&self) -> HashSet<Form> {
        self.terms.iter().map(|t| t.form()).collect()
    }

    /// Extract a copy of the terms in this MultiVector that have the supplied [`Form`]
    pub fn get(&self, c: &Form) -> Option<Vec<Term>> {
        let terms: Vec<Term> = self
            .terms
            .iter()
            .filter(|t| &t.form() == c)
            .cloned()
            .sorted_unstable()
            .collect();

        if !terms.is_empty() {
            Some(terms)
        } else {
            None
        }
    }

    /// Combine together term weights where they have matching Form and Xi
    pub fn simplify(&mut self) {
        self.terms = simplify_terms(self.terms.iter())
    }

    /// Simplify terms and return the resulting [`Multivector`]
    pub fn simplified(self) -> Self {
        Self {
            terms: simplify_terms(self.terms.iter()),
        }
    }
}

fn simplify_terms<'a>(terms: impl Iterator<Item = &'a Term>) -> Vec<Term> {
    // Since we are grouped by summation_key we are safe to unwrap the try_add call without blowing
    // up and we are also guaranteed to have at least one element, so unwrapping the reduce is also
    // fine.
    // TODO: cancelling terms with zero magnitude still needs some thought
    //       John is pretty sure we need some additional checks before it is
    //       safe to drop terms.
    terms
        .sorted_unstable()
        .group_by(|t| t.summation_key())
        .into_iter()
        .flat_map(|(_, group)| group.cloned().reduce(|acc, t| acc.try_add(&t).unwrap()))
        .filter(|t| t.magnitude().is_non_zero())
        .collect()
}

impl fmt::Display for MultiVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = vec![];
        let n_per_line = 6;

        for form in Form::iter() {
            if let Some(terms) = self.get(&form) {
                let form_rows = terms
                    .iter()
                    .map(|t| format!("{}{}{}", t.sign(), t.mag_str(), t.xi_str()))
                    .chunks(n_per_line)
                    .into_iter()
                    .map(|mut c| c.join(", "))
                    .collect_vec();

                if terms.len() < n_per_line + (n_per_line / 2) {
                    rows.push(format!("  a{:<5}( {} )", form, form_rows.join(" ")));
                } else {
                    rows.push(format!("  a{:<5}(", form.to_string()));
                    form_rows
                        .iter()
                        .for_each(|r| rows.push(format!("           {}", r)));
                    rows.push("  )".to_string());
                }
            }
        }

        write!(f, "{{\n{}\n}}", rows.join("\n"))
    }
}

impl IntoIterator for MultiVector {
    type Item = Term;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.terms.into_iter()
    }
}

impl std::iter::FromIterator<Term> for MultiVector {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Term>,
    {
        let mut terms = iter.into_iter().collect_vec();
        terms.sort_unstable();

        MultiVector { terms }
    }
}

impl std::iter::FromIterator<Alpha> for MultiVector {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Alpha>,
    {
        let mut terms = iter.into_iter().map(|a| Term::new(None, a)).collect_vec();
        terms.sort_unstable();

        MultiVector { terms }
    }
}

impl std::iter::FromIterator<MultiVector> for MultiVector {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = MultiVector>,
    {
        iter.into_iter().fold(MultiVector::new(), |a, b| a + b)
    }
}

impl std::iter::Extend<Term> for MultiVector {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Term>,
    {
        self.terms.extend(iter);
        self.terms.sort_unstable();
    }
}

impl std::iter::Extend<MultiVector> for MultiVector {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = MultiVector>,
    {
        for m in iter {
            self.terms.extend(m.terms);
        }
        self.terms.sort_unstable();
    }
}

impl std::iter::Sum<Term> for MultiVector {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Term>,
    {
        iter.fold(MultiVector::new(), |a, b| a + b)
    }
}

impl std::iter::Sum<MultiVector> for MultiVector {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = MultiVector>,
    {
        iter.fold(MultiVector::new(), |a, b| a + b)
    }
}

impl std::iter::Product<MultiVector> for MultiVector {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = MultiVector>,
    {
        if let Some(m) = iter.reduce(|a, b| a * b) {
            m
        } else {
            MultiVector::new()
        }
    }
}

// Supported arithmetic operations:
//   - negation
//   - addition:        term, multivector
//   - subtraction:     term, multivector
//   - multiplication:  uX, iX, magnitude, alpha, term, multivector
//   - division:        uX, iX, magnitude, alpha, term, multivector
//
// TODO (remaining ops):
//   - mul-assign:      uX, iX, magnitude, alpha, term, multivector
//   - div-assign:      uX, iX, magnitude, alpha, term, multivector

macro_rules! __arith_impl {
    (@inner $t:ty: $op:ident $func:ident) => {
        impl ops::$op<$t> for MultiVector {
            type Output = Self;
            fn $func(self, rhs: $t) -> Self::Output {
                let terms = self.terms.clone();
                MultiVector {
                    terms: terms.into_iter().map(|t| ops::$op::$func(t, rhs.clone())).collect()
                }
            }
        }

        impl ops::$op<MultiVector> for $t {
            type Output = MultiVector;
            fn $func(self, rhs: MultiVector) -> Self::Output {
                let terms = rhs.terms.clone();
                MultiVector {
                    terms: terms.into_iter().map(|t| ops::$op::$func(self.clone(), t)).collect()
                }
            }
        }
    };

    ($t:ty) => {
        __arith_impl!(@inner $t: Mul mul);
        __arith_impl!(@inner $t: Div div);
    };
}

__arith_impl!(u8);
__arith_impl!(u16);
__arith_impl!(u32);
__arith_impl!(usize);
__arith_impl!(i8);
__arith_impl!(i16);
__arith_impl!(i32);
__arith_impl!(isize);
__arith_impl!(Magnitude);
__arith_impl!(Alpha);
__arith_impl!(Term);

impl ops::Neg for MultiVector {
    type Output = MultiVector;

    fn neg(self) -> Self::Output {
        MultiVector::from_terms(self.terms.iter().map(|t| -t.clone()).collect())
    }
}

impl ops::Add<Term> for MultiVector {
    type Output = MultiVector;

    fn add(self, rhs: Term) -> Self::Output {
        let mut terms = self.terms;
        terms.push(rhs);

        MultiVector::from_terms(terms)
    }
}

impl ops::Add<MultiVector> for Term {
    type Output = MultiVector;

    fn add(self, rhs: MultiVector) -> Self::Output {
        let mut terms = rhs.terms;
        terms.push(self);

        MultiVector::from_terms(terms)
    }
}

impl ops::Add for MultiVector {
    type Output = MultiVector;

    fn add(self, rhs: MultiVector) -> Self::Output {
        let mut terms = self.terms;
        let mut rhs_terms = rhs.terms;
        terms.append(&mut rhs_terms);

        MultiVector::from_terms(terms)
    }
}

impl ops::Sub for MultiVector {
    type Output = MultiVector;

    fn sub(self, rhs: MultiVector) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Sub<Term> for MultiVector {
    type Output = MultiVector;

    fn sub(self, rhs: Term) -> Self::Output {
        let mut terms = self.terms;
        terms.push(-rhs);

        MultiVector::from_terms(terms)
    }
}

impl ops::Sub<MultiVector> for Term {
    type Output = MultiVector;

    fn sub(self, rhs: MultiVector) -> Self::Output {
        let mut terms = rhs.terms;
        terms.push(-self);

        MultiVector::from_terms(terms)
    }
}

impl ops::Mul for MultiVector {
    type Output = Self;

    fn mul(self, rhs: MultiVector) -> Self::Output {
        full(&self, &rhs)
    }
}

impl ops::Div for MultiVector {
    type Output = Self;

    fn div(self, rhs: MultiVector) -> Self::Output {
        div_by(&self, &rhs)
    }
}

impl From<Alpha> for MultiVector {
    fn from(a: Alpha) -> Self {
        Self {
            terms: vec![Term::new(None, a)],
        }
    }
}

impl From<Vec<Alpha>> for MultiVector {
    fn from(alphas: Vec<Alpha>) -> Self {
        Self {
            terms: alphas.into_iter().map(|a| Term::new(None, a)).collect(),
        }
    }
}

impl From<Vec<&Alpha>> for MultiVector {
    fn from(alphas: Vec<&Alpha>) -> Self {
        Self {
            terms: alphas.into_iter().map(|&a| Term::new(None, a)).collect(),
        }
    }
}

impl From<Term> for MultiVector {
    fn from(t: Term) -> Self {
        Self { terms: vec![t] }
    }
}

impl From<Vec<Term>> for MultiVector {
    fn from(terms: Vec<Term>) -> Self {
        Self { terms }
    }
}

impl From<Vec<&Term>> for MultiVector {
    fn from(terms: Vec<&Term>) -> Self {
        Self {
            terms: terms.into_iter().cloned().collect(),
        }
    }
}
