use std::mem;

use crate::algebra::{
    operations::{ar_product, norm::norm},
    types::{Alpha, Form, Grade, Magnitude, Sign, Term, Zet},
};

/// Something that is usable as an input to AR based calculations
pub trait ArInput {
    /// Decompose self into a Vector of underlying Terms
    fn as_terms(&self) -> Vec<Term>;

    /// Iterate over self, yielding &Term values.
    ///
    /// The default implementation calls `self.as_term()` and then iterates over the resulting
    /// MultiVector: if a more efficient implementation can be provided it should be done so here.
    fn ar_iter(&self) -> ArIterator {
        ArIterator::from_vec(self.as_terms())
    }

    /// Decompose self into a Vector of underlying Alphas
    fn as_alphas(&self) -> Vec<Alpha> {
        self.as_terms().iter().map(|t| t.alpha()).collect()
    }

    /// Check to see if self is entirely composed of scalar elements within
    /// the algebra (i.e. Point: nothing of a higher grade)
    fn is_scalar(&self) -> bool {
        self.ar_iter().all(|t| t.form().grade() == Grade::Zero)
    }

    fn norm(&self) -> Magnitude {
        norm(self.as_terms())
    }
}

/// Something that is producable as an output from AR based calculations
pub trait ArOutput: Sized {
    /// Construct a concrete type from a Vec of Terms
    fn from_terms(terms: Vec<Term>) -> Self;

    /// Construct a concrete type from an iterator of Terms if this can be done in a more efficient
    /// way than using a Vec.
    ///
    /// The default implementation simply collects into a Vec<Term> and then calls through to
    /// from_terms.
    fn from_term_iterator(iter: impl Iterator<Item = Term>) -> Self {
        Self::from_terms(iter.collect())
    }

    /// Construct a concrete type from a Vector of Alphas with default Xi
    /// values derived from each Alpha
    fn from_alphas(alphas: Vec<Alpha>) -> Self {
        Self::from_term_iterator(alphas.iter().map(|a| Term::new(None, *a)))
    }
}

/// Types that implement AR are able to be consumed by any of the library operations
/// provided by arthroprod. The return of these library functions is typically something
/// that also impliments AR:
/// ```
/// # #[macro_use] extern crate arthroprod; fn main() {
/// use arthroprod::algebra::*;
///
/// let a1 = alpha!(0 2 3);
/// let a2 = -alpha!(0 1);
///
/// // full takes two arguments that implement ArInput and tries to return an ArOutput impl.
/// // NOTE: Some ArOutput impls will panic if constructed incorrectly (i.e. constructing an
/// //       alpha from a vector of multiple values)
/// let res_alpha: Alpha = full(&a1, &a2);
/// let res_mvec: MultiVector = full(&a1, &a2);
///
/// assert_eq!(res_alpha, -alpha!(1 2 3));
/// assert_eq!(res_mvec, mvec![-term!(["023", "01"], 1 2 3)]);
/// # }
/// ```
pub trait AR: ArInput + ArOutput + Sized {
    // TODO: inverse is the wrong name for this method
    /// The product inverse through ap of all individual terms
    ///
    /// NOTE: This is _not_ equivalent to the inverse of a MultiVector
    fn inverse(&self) -> Self {
        Self::from_term_iterator(self.ar_iter().map(|t| t.inverse()))
    }

    /// The negation of all alpha values within self
    fn negate(&self) -> Self {
        Self::from_term_iterator(self.ar_iter().map(|t| -(*t).clone()))
    }

    /// Reverse the order basis elements within an object and then resolve back into
    /// permitted Alpha values. In notation, this is denoted with an over tilde (~).
    ///
    /// Even though we only need to carry this operation out for objects of
    /// grade 0 -> 4, we can show that the number of pops required for reversing
    /// an Alpha of grade n is the (n-1)th triangular number.
    fn reversed(&self) -> Self {
        Self::from_term_iterator(self.ar_iter().map(|t| match t.alpha().form().grade() {
            Grade::Two | Grade::Three => -(*t).clone(),
            _ => (*t).clone(),
        }))
    }

    /// The star conjugate of the argument. This negates terms based only on the cancelation of
    /// their indices under the metric.
    fn star(&self) -> Self {
        Self::from_term_iterator(self.ar_iter().map(|t| match t.alpha().form().zet() {
            Zet::A | Zet::E => -(*t).clone(),
            _ => (*t).clone(),
        }))
    }

    /// Implementation of the grade-projection operator <A>n: filter terms, leaving only
    /// those that are of the specified grade. 'grade' is required only to give the
    /// desired output grade, the value of the component passed is ignored.
    fn project(&self, grade: &Form) -> Self {
        Self::from_term_iterator(
            self.ar_iter()
                .filter(|t| {
                    mem::discriminant(&t.form().grade()) == mem::discriminant(&grade.grade())
                })
                .map(|t| (*t).clone()),
        )
    }

    /// Compute the Hermitian conjugate (dagger) of the argument. This has the
    /// effect of negating all terms whos alphas square to -ap.
    ///
    /// The Hermitian Conjugate of a Multivector is defined to be 'a0 ^ rev(M) ^ a0'
    /// with the notation signifying that the product is formed individually for each
    /// term within the MultiVector.
    fn hermitian(&self) -> Self {
        Self::from_term_iterator(self.ar_iter().map(|t| {
            match ar_product(&t.alpha(), &t.alpha()).sign() {
                Sign::Neg => -(*t).clone(),
                Sign::Pos => (*t).clone(),
            }
        }))
    }

    /// Alias for the Hermitian conjugate
    fn dagger(&self) -> Self {
        self.hermitian()
    }

    /// Negate all terms not matching the given [`Form`]
    fn conjugate_form(&self, f: impl AsRef<Form>) -> Self {
        Self::from_term_iterator(self.ar_iter().map(|t| {
            if &t.form() == f.as_ref() {
                (*t).clone()
            } else {
                -(*t).clone()
            }
        }))
    }

    /// Negate all terms with a form not contained the given forms
    fn conjugate_forms(&self, fs: Vec<impl AsRef<Form>>) -> Self {
        let forms: Vec<&Form> = fs.iter().map(|f| f.as_ref()).collect();
        Self::from_term_iterator(self.ar_iter().map(|t| {
            if forms.contains(&&t.form()) {
                (*t).clone()
            } else {
                -(*t).clone()
            }
        }))
    }

    /// Negate all elements not matching the given [`Grade`]
    fn conjugate_grade(&self, g: Grade) -> Self {
        Self::from_term_iterator(self.ar_iter().map(|t| {
            if t.grade() == g {
                (*t).clone()
            } else {
                -(*t).clone()
            }
        }))
    }

    /// Negate all terms with a grade not contained the given grades
    fn conjugate_grades(&self, grades: Vec<Grade>) -> Self {
        Self::from_term_iterator(self.as_terms().iter().map(|t| {
            if grades.contains(&t.grade()) {
                t.clone()
            } else {
                -t.clone()
            }
        }))
    }

    /// The diamond conjugate is defined as `M_diamond = 2<M>0 - M`
    /// It negates everything with a space-time 'direction' (i.e. everything but Point)
    fn diamond(&self) -> Self {
        self.conjugate_grade(Grade::Zero)
    }

    /// The double dagger conjugate is defined as `M_double_dagger = 2<M>2 - M`
    /// It negates everything but the Bivector components (the fields).
    fn double_dagger(&self) -> Self {
        self.conjugate_grade(Grade::Two)
    }

    /// The dual of a Multivector is defined as being '-a0123 ^ M' and is denoted
    /// with an overbar.
    /// It is the inverse of an element through a0123 as opposed to ap, meaning that
    /// the product of an element with its dual is always a0123.
    fn dual(&self) -> Self {
        let q = term!(0 1 2 3);
        Self::from_term_iterator(self.ar_iter().map(|t| q.form_product_with(&t)))
    }
}

#[derive(Clone)]
pub enum ArIterator<'a> {
    Single(std::iter::Once<Term>),
    FromVec(std::vec::IntoIter<Term>),
    FromSlice(std::slice::Iter<'a, Term>),
    FromRefs(std::slice::Iter<'a, &'a Term>),
    SingleRef(std::iter::Once<&'a Term>),
}

#[derive(Clone)]
pub enum RefTerm<'a> {
    Val(Term),
    Ref(&'a Term),
}

impl<'a> std::ops::Deref for RefTerm<'a> {
    type Target = Term;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Val(v) => v,
            Self::Ref(r) => r,
        }
    }
}

impl<'a> ArIterator<'a> {
    pub fn from_term(term: Term) -> Self {
        Self::Single(std::iter::once(term))
    }

    pub fn from_ref_term(term: &'a Term) -> Self {
        Self::SingleRef(std::iter::once(term))
    }

    pub fn from_vec(vec: Vec<Term>) -> Self {
        Self::FromVec(vec.into_iter())
    }

    pub fn from_slice(slice: &'a [Term]) -> Self {
        Self::FromSlice(slice.iter())
    }

    pub fn from_refs(slice: &'a [&'a Term]) -> Self {
        Self::FromRefs(slice.iter())
    }
}

impl<'a> Iterator for ArIterator<'a> {
    type Item = RefTerm<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(it) => it.next().map(RefTerm::Val),
            Self::FromVec(it) => it.next().map(RefTerm::Val),
            Self::FromSlice(it) => it.next().map(RefTerm::Ref),
            Self::FromRefs(it) => it.next().map(|t| RefTerm::Ref(t)),
            Self::SingleRef(it) => it.next().map(RefTerm::Ref),
        }
    }
}

// Provide some simple default impls to avoid the need to wrap things in a full-fat AR
// impl in order to be able to work with them.

impl ArInput for Vec<&Term> {
    fn as_terms(&self) -> Vec<Term> {
        self.iter().map(|&t| t.clone()).collect()
    }

    fn ar_iter(&self) -> ArIterator {
        ArIterator::FromRefs(self.iter())
    }
}

impl ArInput for Vec<Term> {
    fn as_terms(&self) -> Vec<Term> {
        self.clone()
    }

    fn ar_iter(&self) -> ArIterator {
        ArIterator::FromSlice(self.iter())
    }
}

impl AR for Vec<Term> {}

impl ArOutput for Vec<Term> {
    fn from_terms(terms: Vec<Term>) -> Self {
        terms
    }
}

impl ArInput for Vec<&Alpha> {
    fn as_terms(&self) -> Vec<Term> {
        self.iter().map(|a| Term::new(None, **a)).collect()
    }

    fn ar_iter(&self) -> ArIterator {
        ArIterator::FromVec(self.as_terms().into_iter())
    }
}

impl ArInput for Vec<Alpha> {
    fn as_terms(&self) -> Vec<Term> {
        self.iter().map(|a| Term::new(None, *a)).collect()
    }

    fn ar_iter(&self) -> ArIterator {
        ArIterator::FromVec(self.as_terms().into_iter())
    }
}

impl ArOutput for Vec<Alpha> {
    fn from_terms(terms: Vec<Term>) -> Self {
        terms.iter().map(|t| t.alpha()).collect()
    }
}

impl AR for Vec<Alpha> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::{ar_product, Alpha, Form, MultiVector, Term};

    #[test]
    fn hermitian_conjugation_is_correct_for_alphas() {
        for c in Form::iter() {
            let alpha = Alpha::new(Sign::Pos, c);
            let sign = ar_product(&alpha, &alpha).sign();
            let conjugate = alpha.hermitian();

            assert_eq!(conjugate, Alpha::new(sign, c));
        }
    }

    #[test]
    fn hermitian_conjugation_is_correct_for_terms() {
        for c in Form::iter() {
            let alpha = Alpha::new(Sign::Pos, c);
            let sign = ar_product(&alpha, &alpha).sign();
            let term = Term::new(None, alpha);
            let conjugate = term.hermitian();

            assert_eq!(conjugate, Term::new(None, Alpha::new(sign, c)));
        }
    }

    #[test]
    fn hermitian_conjugation_is_correct_for_multivectors() {
        let mut terms: Vec<Term> = vec![];
        let mut negated: Vec<Term> = vec![];

        for c in Form::iter() {
            let alpha = Alpha::new(Sign::Pos, c);
            let sign = ar_product(&alpha, &alpha).sign();
            terms.push(Term::new(None, alpha));
            negated.push(Term::new(None, Alpha::new(sign, c)));
        }

        let conjugate = MultiVector::from_terms(terms).hermitian();
        assert_eq!(conjugate, MultiVector::from_terms(negated));
    }
}
