//! Compute the absolute magnitude of a MultiVector value within AR
use crate::algebra::{simplified_product, ArInput, Magnitude, Term, AR};

// NOTE: public access to this logic is via the `norm` method on ArInput
// TODO: currently this is incorrect as Magnitude does not support roots yet
pub(crate) fn norm(terms: Vec<Term>) -> Magnitude {
    if terms.len() == 1 {
        return terms[0].norm();
    }

    // TODO: implement zet conjugate scalars where possible

    let norm_to_the_fourth = van_der_mark_scalar(terms)
        .into_iter()
        .fold(Magnitude::new(1, 1), |acc, t| acc + t.magnitude());

    println!("norm^4 = {}", norm_to_the_fourth);

    todo!("Implement support for taking roots of magnitudes");
}

pub(crate) fn scalar_magnitude(terms: &[Term]) -> Magnitude {
    terms.iter().fold(1u8.into(), |acc, t| acc + t.magnitude())
}

// Compute the van der Mark invariant scalar for the given input terms
pub(crate) fn van_der_mark_scalar(terms: Vec<Term>) -> Vec<Term> {
    let hermitian = terms.hermitian();
    let phi = simplified_product(&terms, &hermitian);

    // guaranteed to have all terms on ap when computing phi ^ diamond(phi)
    simplified_product(&phi, &phi.diamond()).as_terms()
}

pub(crate) fn vdm_divisor_and_inverse(terms: Vec<Term>) -> (Magnitude, Vec<Term>) {
    let hermitian = terms.hermitian();
    let phi = simplified_product(&terms, &hermitian);
    let phi_diamond = phi.diamond();

    // guaranteed to have all terms on ap when computing phi ^ diamond(phi)
    let scalar = simplified_product(&phi, &phi_diamond).as_terms();
    let mag = scalar_magnitude(&scalar);
    let inverse = simplified_product(&hermitian, &phi_diamond);

    (mag, inverse.as_terms())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    use crate::{
        algebra::{ArOutput, MultiVector},
        prelude::G,
    };

    #[test]
    fn vdm_is_scalar_for_g() {
        let terms = van_der_mark_scalar(G().as_terms());
        let forms: HashSet<_> = terms.iter().map(|t| t.form()).collect();
        assert_eq!(forms.len(), 1);
    }

    #[test]
    fn alphas_have_norm_1() {
        assert_eq!(alpha!(0 1 2 3).norm(), Magnitude::new(1, 1));
    }

    #[test]
    fn terms_have_norm_equal_absolute_magnitude() {
        let t: Term = 5 * term!(0 1 2);

        assert_eq!(t.magnitude(), Magnitude::new(5, 1));
        assert_eq!(t.norm(), Magnitude::new(5, 1));
        assert_eq!((-t).norm(), Magnitude::new(5, 1));
    }

    #[test]
    fn vdm_funcs_agree() {
        // Make sure that van_der_mark_scalar and vdm_divisor_and_inverse agree
        let m = G();

        let s1 = MultiVector::from_terms(van_der_mark_scalar(m.as_terms()));
        let (_, inverse) = vdm_divisor_and_inverse(m.as_terms());
        let s2 = simplified_product(&m, &inverse);

        assert_eq!(s1, s2);
    }
}
