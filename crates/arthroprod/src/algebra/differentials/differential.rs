use std::fmt;

use crate::algebra::{ar_product, Alpha, ArInput, ArOutput, MultiVector, Term, AR};

#[derive(Hash, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Differential {
    // stored in their inverted from so that we can operate from either the left or right
    // by simply using ar_product
    wrt: Vec<Alpha>,
}

impl Differential {
    pub fn new(wrt: &[Alpha]) -> Self {
        Self {
            wrt: wrt.iter().map(|w| w.inverse()).collect(),
        }
    }

    pub fn left_apply(&self, mvec: &MultiVector) -> MultiVector {
        self.apply(mvec, ApplyFrom::Left)
    }

    pub fn right_apply(&self, mvec: &MultiVector) -> MultiVector {
        self.apply(mvec, ApplyFrom::Right)
    }

    fn apply(&self, mvec: &MultiVector, side: ApplyFrom) -> MultiVector {
        MultiVector::from_terms(
            mvec.as_terms()
                .iter()
                .flat_map(|t| {
                    self.wrt
                        .as_alphas()
                        .iter()
                        .map(|w| term_partial(t, w, side))
                        .collect::<Vec<Term>>()
                })
                .collect(),
        )
    }
}

impl fmt::Display for Differential {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ {} }}",
            self.wrt
                .iter()
                .map(|f| format!("{}", f))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[derive(Clone, Copy)]
enum ApplyFrom {
    Left,
    Right,
}

fn term_partial(term: &Term, wrt: &Alpha, side: ApplyFrom) -> Term {
    let a: Alpha = match side {
        ApplyFrom::Left => ar_product(wrt, &term.alpha()),
        ApplyFrom::Right => ar_product(&term.alpha(), wrt),
    };

    let mut t = term.clone();
    t.add_partial(wrt);
    t.set_alpha(a);

    t
}
