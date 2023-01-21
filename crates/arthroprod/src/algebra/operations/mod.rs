//! Algebraic operations on MultiVectors and other AR types.
//!
//! The operations in this module are derived from the Full Product of
//! the algebra, for differential based operations see the [`differentials`]
//! module.

mod ar;
mod ar_product;
mod division;
mod full_product;
pub(crate) mod norm;

pub use self::{
    ar::{ArInput, ArIterator, ArOutput, AR},
    ar_product::{ar_product, init_product_cache},
    division::{div_by, div_into},
    full_product::{full, simplified_product},
};
