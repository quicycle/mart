use std::{fmt, ops};

use crate::algebra::{
    ar_product, ArInput, ArIterator, ArOutput, Form, Grade, Index, Magnitude, Sign, Term, AR,
};

pub(crate) const ALLOWED_ALPHA_STRINGS: [&str; 16] = [
    "p", "23", "31", "12", "0", "023", "031", "012", "123", "1", "2", "3", "0123", "01", "02", "03",
];

/// An Alpha represents a pure element of the algebra without magnitude.
/// It is composed of 0-4 Dimensions with the number of dimensions determining
/// its form: i.e. scalar, vector, bivector, trivector, quadrivector
#[derive(Hash, Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub struct Alpha {
    sign: Sign,
    form: Form,
}

impl Alpha {
    /// Construct a new Alpha value from scratch.
    pub fn new(sign: Sign, form: Form) -> Self {
        Self { sign, form }
    }

    /// Allow or construction of Alpha values from a dynamically created vector of
    /// [`Index`] values. Errors if the given vector does not map to one of the allowed
    /// forms given in [`ALLOWED_ALPHA_FORMS`].
    pub fn try_from_indices(sign: Sign, indices: &[Index]) -> Result<Alpha, String> {
        let form = Form::try_from_indices(indices)?;

        Ok(Alpha::new(sign, form))
    }

    /// Take a copy of this Alphas [`Form`]
    pub fn form(&self) -> Form {
        self.form
    }

    /// Take a copy of this Alphas [`Grade`]
    pub fn grade(&self) -> Grade {
        self.form.grade()
    }

    /// Take a copy of this Alphas [`Sign`]
    pub fn sign(&self) -> Sign {
        self.sign
    }
}

impl fmt::Display for Alpha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}a{}", self.sign, self.form)
    }
}

impl AsRef<Form> for Alpha {
    fn as_ref(&self) -> &Form {
        &self.form
    }
}

impl ArInput for Alpha {
    fn as_terms(&self) -> Vec<Term> {
        vec![Term::new(None, *self)]
    }

    fn as_alphas(&self) -> Vec<Alpha> {
        vec![*self]
    }

    fn ar_iter(&self) -> ArIterator {
        ArIterator::from_term(Term::new(None, *self))
    }

    fn norm(&self) -> Magnitude {
        Magnitude::new(1, 1)
    }
}

impl ArOutput for Alpha {
    fn from_terms(terms: Vec<Term>) -> Self {
        if terms.len() != 1 {
            panic!("Can only construct an Alpha from a single term")
        };

        terms[0].alpha()
    }

    fn from_alphas(alphas: Vec<Alpha>) -> Self {
        if alphas.len() != 1 {
            panic!("Can only construct an Alpha from a single value")
        };

        alphas[0]
    }
}

impl AR for Alpha {
    fn inverse(&self) -> Self {
        Alpha {
            sign: self.sign.combine(ar_product(self, self).sign),
            form: self.form,
        }
    }
}

// Supported arithmetic operations:
//   - negation
//   - multiplication:  alpha
//   - division:        alpha
//   - mul-assign:      alpha
//   - div-assign:      alpha

impl ops::Neg for Alpha {
    type Output = Alpha;

    fn neg(self) -> Self::Output {
        Alpha {
            sign: -self.sign,
            form: self.form,
        }
    }
}

impl ops::Mul<Alpha> for Alpha {
    type Output = Self;

    fn mul(self, rhs: Alpha) -> Self::Output {
        ar_product(&self, &rhs)
    }
}

impl ops::Div<Alpha> for Alpha {
    type Output = Self;

    fn div(self, rhs: Alpha) -> Self::Output {
        ar_product(&self, &rhs.inverse())
    }
}

impl ops::MulAssign for Alpha {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl ops::DivAssign for Alpha {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_strings_are_correct() {
        let forms: Vec<String> = Form::iter().map(|f| format!("{}", f)).collect();
        assert_eq!(forms, ALLOWED_ALPHA_STRINGS);
    }
}
