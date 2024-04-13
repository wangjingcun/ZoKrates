// see https://github.com/mcarton/rust-derivative/issues/115
#![allow(clippy::non_canonical_partial_ord_impl)]

pub mod common;
pub mod flat;
pub mod ir;
pub mod typed;
pub mod untyped;
pub mod zir;

pub use common::Solver;
