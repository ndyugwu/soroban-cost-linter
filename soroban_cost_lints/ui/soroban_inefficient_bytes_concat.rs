//! UI test fixture for the `soroban_inefficient_bytes_concat` lint.
//!
//! This module tests both positive cases (triggering warnings when `Bytes` methods
//! like `push_back` are invoked repeatedly inside loops) and negative/allowed
//! cases (such as small, bounded loops where the performance impact is negligible).

#![warn(soroban_inefficient_bytes_concat)]

/// Mock implementation of the Soroban SDK types used for static analysis testing.
pub mod soroban_sdk {
    pub struct Bytes;
    impl Bytes {
        /// Appends a single 32-bit integer to the end of the `Bytes` container.
        pub fn push_back(&mut self, _val: u32) {}
        /// Appends another `Bytes` container to this one.
        pub fn append(&mut self, _other: &Bytes) {}
    }
}
use soroban_sdk::Bytes;

/// Positive test case: repeatedly pushing back to a `Bytes` container inside a loop.
/// This pattern incurs high memory allocation and CPU instruction costs and triggers
/// the `soroban_inefficient_bytes_concat` lint warning.
fn bad_push_back(mut b: Bytes) {
    for _ in 0..10 {
        b.push_back(1); //~ WARNING inefficient Bytes concatenation inside a loop
    }
}

/// Negative/Allowed test case: small, provably bounded loop using `#[allow(...)]`.
/// False positive: the loop bounds are small and fixed, so the allocation cost is
/// negligible, but the structural pattern would trigger the lint without explicit allow suppression.
#[allow(soroban_inefficient_bytes_concat)]
fn good_small_push_back(mut b: Bytes) {
    // False positive: loop is small and provably bounded, so cost is negligible,
    // but lint flags it anyway unless allowed.
    for _ in 0..2 {
        b.push_back(1);
    }
}

fn main() {}
