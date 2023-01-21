//! Given that we are working within a non-commutative algebera, we need to define a convention
//! for division to establish the divisor. In practice we are chosing between 1/A ^ B or A ^ 1/B.
//! Our current thinking is that we need to define division as the former (dividing A into B).
use crate::algebra::{
    full, operations::norm::vdm_divisor_and_inverse, ArInput, ArOutput, MultiVector, AR,
};

/// Divide left into right.
///
/// When left and right are both terms or alphas, this is a relatively simple inversion of left and
/// then forming the full product. For MultiVectors this requires a full general inverse using the
/// Van Der Mark
pub fn div_into<L, R, T>(left: &L, right: &R) -> T
where
    L: ArInput,
    R: ArInput,
    T: ArOutput,
{
    div(left, right, ApplyFrom::Left)
}

/// Divide right into left.
///
/// When left and right are both terms or alphas, this is a relatively simple inversion of left and
/// then forming the full product. For MultiVectors this requires a full general inverse using the
/// Van Der Mark
pub fn div_by<L, R, T>(left: &L, right: &R) -> T
where
    L: ArInput,
    R: ArInput,
    T: ArOutput,
{
    div(left, right, ApplyFrom::Right)
}

enum ApplyFrom {
    Left,
    Right,
}

fn div<L, R, T>(left: &L, right: &R, side: ApplyFrom) -> T
where
    L: ArInput,
    R: ArInput,
    T: ArOutput,
{
    let mut lterms = left.as_terms();
    let mut rterms = right.as_terms();

    T::from_terms(if lterms.len() == 1 && rterms.len() == 1 {
        let (left, right) = match side {
            ApplyFrom::Left => (lterms[0].inverse(), rterms.remove(0)),
            ApplyFrom::Right => (lterms.remove(0), rterms[0].inverse()),
        };

        vec![left.form_product_with(&right)]
    } else {
        let (divisor, left, right) = match side {
            ApplyFrom::Left => {
                let (divisor, left) = vdm_divisor_and_inverse(lterms);
                (divisor, left, rterms)
            }

            ApplyFrom::Right => {
                let (divisor, right) = vdm_divisor_and_inverse(rterms);
                (divisor, lterms, right)
            }
        };

        let product: MultiVector = full(&left, &right);
        (product / divisor).as_terms()
    })
}
