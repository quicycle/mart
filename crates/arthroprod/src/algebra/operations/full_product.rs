use itertools::Itertools;

use crate::algebra::{ArInput, ArOutput, MultiVector};

// TODO: Change this all so that inputs are always MultiVectors.
//       Outputs can still be AROutput but everything else should use impl From<T> for MultiVector

/// The full product between two elements within AR is defined as an extension of the traditional
/// Clifford product from a Clifford Algebera: we form the Cartesian product of the terms in left
/// and right using the AR full product.
pub fn full<L, R, T>(left: &L, right: &R) -> T
where
    L: ArInput,
    R: ArInput,
    T: ArOutput,
{
    // T::from_term_iterator(
    //     left.as_terms()
    //         .iter()
    //         .cartesian_product(right.as_terms().iter())
    //         .map(|(l, r)| l.form_product_with(r)),
    // )
    T::from_term_iterator(
        left.ar_iter()
            .cartesian_product(right.ar_iter())
            .map(|(l, r)| l.form_product_with(&r)),
    )
}

/// Force simplification of a resultant [`MultiVector`] when computing a product using [`full`]
pub fn simplified_product<L, R>(left: &L, right: &R) -> MultiVector
where
    L: ArInput,
    R: ArInput,
{
    full::<L, R, MultiVector>(left, right).simplified()
}
