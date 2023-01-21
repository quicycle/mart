//! This is a demonstration of the equivalence of dA as F when computing
//! force equations from the four-current.
#![allow(non_snake_case)]

use arthroprod::{
    algebra::{full, MultiVector},
    prelude::*,
    term,
};

fn main() {
    let d = Dmu();
    let A = A() + term!(0);
    let dA = d.left_apply(&A);
    let ddA = d.left_apply(&dA);
    let mut FdF: MultiVector = full(&dA, &ddA);
    FdF.simplify();

    let f = Even_sub_algebra();
    let mut FdF_2: MultiVector = full(&f, &d.left_apply(&f));
    FdF_2.simplify();

    println!("d = {}\n", d);
    println!("A = {}\n", A);
    println!("dA = F = {}\n", dA);
    println!("ddA = dF = {}\n", ddA);
    println!("dAddA = FdF = {}\n", FdF);
    println!("FDF from standard F = {}", FdF_2)
}
