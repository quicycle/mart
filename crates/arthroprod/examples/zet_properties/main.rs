#[macro_use]
extern crate arthroprod;

use arthroprod::algebra::{full, MultiVector, AR};
use arthroprod::prelude::*;

type NamedZets<'a> = Vec<(&'a str, MultiVector)>;

fn sign_checked(m: &MultiVector) -> String {
    m.iter().map(|t| t.sign().to_string()).collect()
}

fn for_all_zets(name: &str, f: impl Fn(&MultiVector) -> String, zets: &NamedZets) {
    println!("{}", name);
    for (zet, mvec) in zets.iter() {
        println!("  {}: {}", zet, f(mvec));
    }
    println!();
}

fn main() {
    let zets = vec![
        ("ZB", Zet_B()),
        ("ZT", Zet_T()),
        ("ZA", Zet_A()),
        ("ZE", Zet_E()),
    ];

    let a0 = alpha!(0);
    let hermitian = |m: &MultiVector| sign_checked(&m.hermitian());
    let dual = |m: &MultiVector| sign_checked(&m.dual());
    let reversed = |m: &MultiVector| sign_checked(&m.reversed());
    let herm_dual = |m: &MultiVector| (m.hermitian() == m.reversed()).to_string();
    let parity = |m: &MultiVector| sign_checked(&full::<_, MultiVector, _>(&a0, &full(m, &a0)));

    for_all_zets("Hermitian conjugate", hermitian, &zets);
    for_all_zets("Dual multivector", dual, &zets);
    for_all_zets("Reversed indices", reversed, &zets);
    for_all_zets("Hermitian == reversed", herm_dual, &zets);
    for_all_zets("Parity: a0_Z_a0", parity, &zets);
}
