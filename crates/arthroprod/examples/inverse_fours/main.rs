#[macro_use]
extern crate arthroprod;

use arthroprod::{
    algebra::{full, Form, MultiVector, AR},
    prelude::*,
};

/// Helper for forming products and simplifying the resulting terms
fn simplified_product(m: &MultiVector, f: impl Fn(&MultiVector) -> MultiVector) -> MultiVector {
    let res: MultiVector = full(m, &f(m));
    res.simplified()
}

/// Display the alpha values contained within a multivector
fn simple_form_rep(m: &MultiVector) -> String {
    let mut forms: Vec<Form> = m.iter().map(|t| t.form()).collect();
    forms.sort_unstable();
    forms.dedup();

    forms.iter().map(|f| format!("a{} ", f)).collect()
}

/// Print a MultiVector if it is a pure scalar, otherwise list the forms it is composed of
fn print_if_scalar(name: &str, m: MultiVector) {
    if m.is_scalar() {
        println!("{}: {}", name, m)
    }
}

fn main() {
    // MultiVector product functions
    let squared = |m: &MultiVector| simplified_product(m, |n| n.clone());
    let phi = |m: &MultiVector| simplified_product(m, |n| n.hermitian());
    let ddaggered = |m: &MultiVector| simplified_product(m, |n| n.double_dagger());
    let vdm_scalar = |m: &MultiVector| simplified_product(&phi(m), |n| n.diamond());

    let time_like = vec![term!(), term!(0), term!(1 2 3), term!(0 1 2 3)];
    let space_like = vec![B(), T(), A(), E()];

    for t in time_like.iter() {
        for s in space_like.iter() {
            let mvec = mvec![s, t];

            println!("MultiVector = {{ {} }}", simple_form_rep(&mvec));
            print_if_scalar("squared", squared(&mvec));
            print_if_scalar("phi", phi(&mvec));
            print_if_scalar("double_dagger", ddaggered(&mvec));
            print_if_scalar("VdM_scalar", vdm_scalar(&mvec));
            println!("\n");
        }
    }
}
