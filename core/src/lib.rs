//! Main entry point for the Dynamath core library.

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

pub fn hello() -> &'static str {
    "Dynamath core library loaded: dynamic mathematics of ideas."
}
