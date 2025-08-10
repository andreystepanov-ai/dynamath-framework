//! Core library for Dynamic Mathematics (Dynamath).
//! This module exposes key submodules: ontypes, space, flow, graphdyn, energy,
//! entropy, phase, fields, tensor and ffi. Each module encodes a part of the
//! Dynamath formalism described in the paper.
//!
//! The `hello()` function returns a friendly string to demonstrate that the
//! library is linked correctly.

pub mod ontypes;
pub mod space;
pub mod flow;
pub mod graphdyn;
pub mod energy;
pub mod entropy;
pub mod phase;
pub mod fields;
pub mod tensor;
pub mod ffi;

/// Returns a greeting from the Dynamath core.
pub fn hello() -> String {
    "Hello from dynamath-core!".to_string()
}
