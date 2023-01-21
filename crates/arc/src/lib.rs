//! Placeholder to allow for multiple programs with the same dependencies in the examples directory

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate arthroprod;

use arthroprod::algebra::{MultiVector, Term};
use arthroprod::prelude::{A, B, E, T};
use arthroprod::*;

pub mod display;
pub mod exec;
pub mod inverses;

/// A function between multivectors.
///
/// Used to allow for iterating over a set of functions
pub type MvecFunc = dyn Fn(&MultiVector) -> MultiVector;

/// All singlet elements from the algebra (pthq)
pub fn singlets() -> Vec<Term> {
    vec![term!(), term!(0), term!(1 2 3), term!(0 1 2 3)]
}

/// All triads from the algebr (BTAE)
pub fn triads() -> Vec<MultiVector> {
    vec![B(), T(), A(), E()]
}

#[doc(hidden)]
#[macro_export]
macro_rules! test_cases {
    {
        $test_name:ident;
        args: ( $($arg:ident: $t:ty),* $(,)? );

        $(
            case: $case_name:ident => ( $($param:expr),* );
        )+
        body: $body:expr
    } => {
        paste::paste! {
            fn [<$test_name _helper>]($($arg: $t),*) {
                $body
            }
        }

        $(
            paste::paste! {
                #[test]
                fn [<$test_name _ $case_name>]() {
                    [<$test_name _helper>]($($param),*)
                }
            }
        )+
    };
}
