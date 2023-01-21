//! Parsing and evaluation of *.arp files
//!
//! The simple *.arp file format allows for writing quick scripts to describe calculations without
//! having to have a Rust toolchain installed or write any Rust code directly. The trade off is
//! that you will not be able to perform arbitrary operations on values as you would with the full
//! arthroprod API but it should be sufficient for most "back of the envelope" calculations you may
//! want to do as part of normal calculation.
//!
//! ## Supported Values
//! ### Alphas
//! - Terms
//! - Multivectors
//! - Differential operators

// TODO:
//   - control output reprs (pthq/BTAE, simplify etc)

mod eval;
mod repl;

pub use eval::{EvalError, Evaluator, Result};
pub use repl::repl;
