//! Arbitrary place hexadecimal digits viewer of pi.
//!
//! This crate computes the hexadecimal digits of pi at any place directly,
//! without first calculating the preceding digits, using either of two formulas:
//!
//! - [`bbp::pihex`] - the Bailey-Borwein-Plouffe formula.
//! - [`bellard::pihex`] - Bellard's formula, which converges faster.
//!
//! Both functions return the same four hexadecimal digits for a given place.
//!
//! ```
//! assert_eq!(pihex::bbp::pihex(100), "29b7");
//! assert_eq!(pihex::bellard::pihex(100), "29b7");
//! ```
//!
//! # Accuracy
//!
//! The digits are extracted from double-precision floating-point sums, so the
//! results are not guaranteed to be correct. A place whose binary expansion of
//! pi holds a long run of zeros (more than 53 bits, i.e. over 13 successive `0`
//! or `f` hexadecimal digits) may be computed incorrectly.

pub mod bbp;
pub mod bellard;
mod util;
