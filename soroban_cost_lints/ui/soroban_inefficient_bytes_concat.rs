//! UI test fixture for the `soroban_inefficient_bytes_concat` lint.
//!
//! This module tests detection of inefficient byte-concatenation patterns
//! (such as repeated `.push_back()` or manual appends) inside loops within
//! Soroban smart contracts. Such patterns cause excessive CPU instruction
//! consumption and memory growth relative to batch operations.

#![warn(soroban_inefficient_bytes_concat)]

/// Mock Soroban SDK types and interfaces used for testing the bytes concatenation lint.
pub mod soroban_sdk {
    /// Mock Soroban SDK `Bytes` type representing a byte buffer.
    pub struct Bytes;
    impl Bytes {
        /// Appends a single byte or value to the buffer. Doing this repeatedly
        /// in a loop incurs heavy metering overhead.
        pub fn push_back(&mut self, _val: u32) {}

        /// Appends another buffer in bulk, which is much more efficient.
        pub fn append(&mut self, _other: &Bytes) {}
    }
}
use soroban_sdk::Bytes;

/// Triggers warning: repeated single-element pushes inside a dynamic or
/// non-trivial loop lead to high CPU and memory resource metering costs.
fn bad_push_back(mut b: Bytes) {
    for _ in 0..10 {
        b.push_back(1); //~ WARNING inefficient Bytes concatenation inside a loop
    }
}

/// Exempted via attribute: small, provably bounded loops have negligible
/// cost impact, allowing developers to bypass the lint when performance is
/// unaffected.
#[allow(soroban_inefficient_bytes_concat)]
fn good_small_push_back(mut b: Bytes) {
    // False positive: loop is small and provably bounded, so cost is negligible,
    // but lint flags it anyway unless allowed.
    for _ in 0..2 {
        b.push_back(1);
    }
}

/// Entry point executing the test cases to verify lint emission and suppression.
fn main() {}
