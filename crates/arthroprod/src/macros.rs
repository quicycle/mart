//! Utility macros for constructing algebraic structures to manipulate. All of these will panic if
//! given invalid indices to work with so make sure to initialise your data early on in your
//! program.

/// Quick creation of raw indices
#[macro_export]
macro_rules! index(
    (0) => { $crate::algebra::Index::Zero };
    (1) => { $crate::algebra::Index::One };
    (2) => { $crate::algebra::Index::Two };
    (3) => { $crate::algebra::Index::Three };
);

/// Quick creation of bare forms
#[macro_export]
macro_rules! form(
    ($($num:tt)*) => {
        {
            #[allow(unused_mut, clippy::vec_init_then_push)]
            let ixs = {
                let mut ixs = Vec::new();
                $(ixs.push(index!($num));)*
                ixs
            };

            $crate::algebra::Form::try_from_indices(&ixs).unwrap()
        }
    };
);

/// Simpler variadic generation of [`Alpha`] values.
/// This is the recommended way of creating raw alpha values if they are needed. Arguments
/// are u8s in the range 0-3.
#[macro_export]
macro_rules! alpha(
    ($($num:tt)*) => {
        {
            let sign = $crate::algebra::Sign::Pos;
            #[allow(unused_mut, clippy::vec_init_then_push)]
            let ixs = {
                let mut ixs = Vec::new();
                $(ixs.push(index!($num));)*
                ixs
            };

            $crate::algebra::Alpha::try_from_indices(sign, &ixs).unwrap()
        }
    };
);

/// Simpler variadic generation of [`Term`] values.
/// Terms created this way will have a default value (if one is not provided) and a
/// magnitude of 1. See [`alpha`] for more information on how the underlying [`Alpha`]
/// value is generated. It is also possible to specify a set of [`Xi`] values to use
/// for the term by providing a list of strings to use as the Xi symbolic values.
#[macro_export]
macro_rules! term(
    ($($num:expr)*) => {
        {
            let sign = $crate::algebra::Sign::Pos;
            #[allow(unused_mut, clippy::vec_init_then_push)]
            let ixs = {
                let mut ixs = Vec::new();
                $(ixs.push($crate::algebra::Index::try_from_u8($num).unwrap());)*
                ixs
            };
            let alpha = $crate::algebra::Alpha::try_from_indices(sign, &ixs).unwrap();

            $crate::algebra::Term::new(None, alpha)
        }
    };

    ([$($xi:expr),+], $($num:expr) *) => {
        {
            let sign = $crate::algebra::Sign::Pos;
            #[allow(unused_mut, clippy::vec_init_then_push)]
            let (ixs, xis) = {
                let mut ixs = Vec::new();
                let mut xis = Vec::new();
                $(xis.push($xi);)+
                $(ixs.push($crate::algebra::Index::try_from_u8($num).unwrap());)*
                (ixs, xis)
            };
            let alpha = $crate::algebra::Alpha::try_from_indices(sign, &ixs).unwrap();

            $crate::algebra::Term::from_xis_and_alpha(xis, alpha)
        }
    };

    ($sym:tt, $($num:expr) +) => {
        {
            let sign = $crate::algebra::Sign::Pos;
            #[allow(unused_mut, clippy::vec_init_then_push)]
            let ixs = {
                let mut ixs = Vec::new();
                $(ixs.push($crate::algebra::Index::try_from_u8($num).unwrap());)+
                ixs
            };
            let alpha = $crate::algebra::Alpha::try_from_indices(sign, &ixs).unwrap();

            $crate::algebra::Term::new(Some($sym), alpha)
        }
    };
);

/// Simpler variadic generation of [`MultiVector`] values.
/// Each argument must impliment the AR trait so that it is possible to convert them to
/// [`Term`]s, with the resulting MultiVector is the sum of all terms generated this way.
#[macro_export]
macro_rules! mvec(
    [$($ar_elem:expr),+] => {
        {
            let mut terms = Vec::new();
            $(terms.extend($ar_elem.as_terms());)+

            $crate::algebra::MultiVector::from_terms(terms)
        }
    };
);

/// A simple helper for constructing hashmaps with less verbosity.
/// # Examples
///
/// ```
/// # #[macro_use] extern crate arthroprod; fn main() {
/// use std::collections::HashMap;
///
/// let m = map!{
///     "foo" => vec![1, 2, 3],
///     "bar" => vec![4, 5, 6]
/// };
///
/// assert_eq!(m.get("foo"), Some(&vec![1, 2, 3]));
/// # }
/// ```
#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ $(,)? } => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(_map.insert($key, $value);)+
            _map
        }
    };
);

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
