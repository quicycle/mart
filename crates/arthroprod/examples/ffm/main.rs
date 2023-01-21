//! This is intended as a test of the current capabilities of arthroprod through looking
//! at the products and differentials formed by Martin in the DAppendix 'paper'.
//! My notes (and attempts to re-compute / translate notation and follow along) start
//! on page 204 of my first day book (army green).
//! In my notes I refer to the input multivector here as F_pq meaning the Faraday tensor
//! along with p (ap) and q (a0123). In arthroprod I'm giving this the more conventional
//! name of "Even_sub_algebra".
//!
//! I like this set of results as it neatly demonstrates something about the 'colour' of
//! multivectors and operators and the results of forming products and computing differentials.
//! John talks about this in terms of generating odd from even etc, which is directly
//! analagous but I tend to think in terms of blue/green -> even and red/purple -> odd.
//!
use arthroprod::algebra::{full, Differential, MultiVector, AR};
use arthroprod::prelude::*;

/// Almost everything cancels with this one: it should have terms on a_p and a_0i
fn energy_momentum_density(f_pq: &MultiVector) {
    let em_density: MultiVector = full(f_pq, &f_pq.hermitian());

    println!("[EM Density] Fpq ^ Fpq! = {}\n", em_density.simplified());
}

/// There is less cancelation in this one but we should still remain on the even
/// sub algebra with the resultant multivector (potentially dropping the a_0i and
/// a_p terms due to internal cancelation depending on the magnitudes in the original
/// multivector: it should have terms covering all of F_pq
fn lagrangian_density(f_pq: &MultiVector) {
    let l_density: MultiVector = full(f_pq, f_pq);

    println!(
        "[Lagrangian Density] Fpq ^ Fpq = {}\n",
        l_density.simplified()
    );
}

/// This will really test the cancelation logic: we should end up with (effectively)
/// 2 x the em_density but dropping the Poynting vector term of ExB.
fn stress_tensor(f_pq: &MultiVector) {
    let left: MultiVector = full(f_pq, &f_pq.hermitian());
    let right: MultiVector = full(&f_pq.hermitian(), f_pq);

    println!(
        "[Stress Tensor] (Fpq ^ Fpq!) + (Fpq! ^ Fpq) = {}\n",
        (left + right).simplified()
    );
}

/// This is simply applying the standard differential operator Dmu to Fpq which has
/// the effect of mapping us from Even -> Odd. As I've mentioned in the past, the
/// structure (Maxwell) that we get out of this is actually a lot more predictable
/// when you realise that while Dmu is a conventional operator (space-time) it is
/// actually mixing elements from two Zets which are the 'natural' decomposition of
/// products within AR. If we compute the _full_ derivative, DG, of a multivector
/// then we see the repeated structure that arrises from the nested Zet structure
/// of the algebra we are using.
fn maxwell(d: &Differential, f_pq: &MultiVector) {
    println!("[Extended Maxwell] Dmu Fpq = {}\n", d.left_apply(f_pq))
}

/// This one should be good for testing cancelation now that we have nested products
/// and differentials. Without implimenting the term simplification logic that is
/// the core purpose of arthroprod (and extension over arpy) this one is going to
/// be pretty messy...)
fn lorentz_force_density(d: &Differential, f_pq: &MultiVector) {
    let m = d.left_apply(f_pq);
    let lf_density: MultiVector = full(f_pq, &m);

    println!(
        "[Lorentz Force Density] Fpq Dmu Fpq = {}\n",
        lf_density.simplified()
    );
}

fn main() {
    let f_pq = Even_sub_algebra(); // Even
    let d = Dmu(); // Odd

    println!("[Even Sub-Algebra] Fpq = {}\n", f_pq);
    energy_momentum_density(&f_pq); // Even Even -> Even
    lagrangian_density(&f_pq); // Even Even -> Even
    stress_tensor(&f_pq); // Even Even -> Even
    maxwell(&d, &f_pq); // Odd Even -> Odd
    lorentz_force_density(&d, &f_pq); // Even Odd Even -> Odd
}
