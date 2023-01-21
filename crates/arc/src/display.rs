use std::collections::HashSet;

use itertools::Itertools;

use arthroprod::algebra::{Form, MultiVector};
use arthroprod::*;

/// Display the given multivector using pthq/BTAE shorthand notation
pub fn shorthand_rep(m: &MultiVector) -> String {
    let mut elems: Vec<&str> = vec![];

    macro_rules! __check_for {
        ($s:expr, $a:expr) => {
            if m.get(&$a.form()).is_some() {
                elems.push($s);
            }
        };
    }

    __check_for!("p", alpha!());
    __check_for!("t", alpha!(0));
    __check_for!("h", alpha!(1 2 3));
    __check_for!("q", alpha!(0 1 2 3));
    __check_for!("B", alpha!(2 3));
    __check_for!("T", alpha!(0 2 3));
    __check_for!("A", alpha!(1));
    __check_for!("E", alpha!(0 1));

    elems.join("")
}

/// Display the alpha values contained within a multivector
pub fn simple_form_rep(m: &MultiVector) -> String {
    let forms: HashSet<Form> = m.iter().map(|t| t.form()).collect();
    format!(
        "{{ {} }}",
        forms.iter().sorted().map(|f| format!("a{}", f)).join(" ")
    )
}
