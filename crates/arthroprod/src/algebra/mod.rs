//! Data structures, functions and differential operators for performing calculations
//! within the framework of Absolute Relativity.

pub mod differentials;
pub mod operations;
pub mod types;

pub use self::{differentials::*, operations::*, types::*};
