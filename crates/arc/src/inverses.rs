use itertools::Itertools;

use arthroprod::algebra::{full, MultiVector, AR};
use arthroprod::prelude::*;

/// Conjugates for a given multivector that may result in a scalar output.
///
/// If desired, all candidates can be computed (whether they result in a scalar or not),
/// alternatively it is possible to select the lowest order / simplest product that can be used to
/// map the input multivector to a scalar. Typically this is used when computing divisions but it
/// is also useful to look at independently in certain situations (such as the analysis of null
/// hyperplanes).
pub struct ScalarCandidates {
    mvec: MultiVector,
    // The potential scalars
    square: MultiVector,
    dagger: MultiVector,
    diamond: MultiVector,
    zet: MultiVector,
    van_der_mark: MultiVector,
}

impl ScalarCandidates {
    pub fn new(m: &MultiVector) -> Self {
        fn simplified_product(m1: &MultiVector, m2: &MultiVector) -> MultiVector {
            full::<_, _, MultiVector>(m1, m2).simplified()
        }

        Self {
            mvec: m.clone(),
            square: simplified_product(m, m),
            dagger: simplified_product(m, &m.dagger()),
            diamond: simplified_product(m, &m.diamond()),
            zet: simplified_product(m, &conjugate_zet(m)),
            van_der_mark: {
                let phi = simplified_product(m, &m.dagger());
                simplified_product(&phi, &phi.diamond())
            },
        }
    }

    /// Iterate over the named cadidates in order of complexity
    pub fn iter(&self) -> ScalarCandidatesIterator {
        ScalarCandidatesIterator {
            ix: 0,
            candidates: self,
        }
    }

    /// Iterate over candidates that are scalars
    pub fn iter_scalars(&self) -> impl Iterator<Item = (&str, &MultiVector)> {
        ScalarCandidatesIterator {
            ix: 0,
            candidates: self,
        }
        .filter(|(_, c)| c.is_scalar())
    }

    /// The original multivector that these scalar candidates have been computed from
    pub fn original(&self) -> &MultiVector {
        &self.mvec
    }

    /// The simplest scalar possible for the input multivector.
    pub fn simplest(&self) -> (&str, &MultiVector) {
        // vdM is guaranteed to be a scalar so unwrap is fine
        self.iter_scalars().next().unwrap()
    }
}

pub struct ScalarCandidatesIterator<'a> {
    ix: usize,
    candidates: &'a ScalarCandidates,
}

impl<'a> Iterator for ScalarCandidatesIterator<'a> {
    type Item = (&'a str, &'a MultiVector);

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.ix {
            0 => Some(("sqr", &self.candidates.square)),
            1 => Some(("dgr", &self.candidates.dagger)),
            2 => Some(("dmd", &self.candidates.diamond)),
            3 => Some(("zet", &self.candidates.zet)),
            4 => Some(("vdm", &self.candidates.van_der_mark)),
            _ => None,
        };

        self.ix += 1;

        item
    }
}

/// A scalar multivector will have a null hyperplane if there are both positive
/// and negative elements in its expression
pub fn null_hyperplane(m: &MultiVector) -> bool {
    !m.iter().map(|t| t.sign()).all_equal()
}

// XXX: this only works for inputs with a full single triad
//      a real impl is going to need the modified form internals
//      allowing for talking about Zets (which would then be passed as an arg)
fn conjugate_zet(m: &MultiVector) -> MultiVector {
    let (p, t, h, q) = (alpha!(), alpha!(0), alpha!(1 2 3), alpha!(0 1 2 3));
    if m.get(&form!(2 3)).is_some() {
        m.conjugate_forms(vec![p, t, h, q])
    } else if m.get(&form!(0 2 3)).is_some() {
        m.conjugate_forms(vec![p, t])
    } else if m.get(&form!(1)).is_some() {
        m.conjugate_forms(vec![p, h])
    } else if m.get(&form!(0 1)).is_some() {
        m.conjugate_forms(vec![p, q])
    } else {
        m.clone()
    }
}
