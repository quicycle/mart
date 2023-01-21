#[macro_use]
extern crate arthroprod;

use arthroprod::algebra::{full, Form, MultiVector, Sign, AR};
use arthroprod::prelude::*;

fn main() {
    let m = mvec![term!(0 1 2 3), term!(2 3), term!(3 1), term!(1 2)];
    let mres = full::<_, _, MultiVector>(&m, &m.double_dagger()).simplified();

    println!("m = {}", m);
    println!("m ^ m_ddagger = {}\n", mres);

    let res = Dmu().left_apply(&G());
    println!("Dmu G = {}\n", res);

    for form in Form::iter() {
        if let Some(terms) = res.get(&form) {
            let signs = terms
                .iter()
                .map(|t| if t.sign() == Sign::Pos { "+" } else { "-" })
                .collect::<Vec<&str>>()
                .join("");
            println!("{} {}", signs, terms[0].form());
        }
    }
}
