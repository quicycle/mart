//! Data types used to drive computations within AR.
//!
//! While most of the arthroprod library functions will accept anything that impliments
//! the [`AR`] trait, the primary unit of computation is the [`MultiVector`]. This is in
//! keeping with the way that we work by hand, with [`Term`] and [`Alpha`] values being
//! considered rarely. That said, in some cases we make use of "raw" Alphas to implement
//! certain conjugates and commutators.

// NOTE: Arithmetic operation impls are organised from the POV of the operand with the greatest
//       amount of structure:
//
//         Magnitude > Alpha > Term > MultiVector
//
//       i.e. you will find both "Term x Magnitude" and "Magnitude x Term" in term.rs
//
//       For structs other than Alpha, impls are given for all uX integer variants.
//       For structs other than Alpha and Magnitude, impls are given for all iX integer variants.

mod alpha;
mod enums;
mod form;
mod magnitude;
mod multivector;
mod term;
mod xi;

pub use self::alpha::Alpha;
pub(crate) use self::alpha::ALLOWED_ALPHA_STRINGS;
pub use self::enums::{Grade, Index, Sign};
pub use self::form::{Form, Orientation, Zet};
pub use self::magnitude::Magnitude;
pub use self::multivector::MultiVector;
pub use self::term::Term;
pub use self::xi::Xi;
