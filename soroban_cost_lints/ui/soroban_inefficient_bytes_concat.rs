//! # UI Test: `soroban_inefficient_bytes_concat`
//!
//! This UI test thoroughly verifies that the `soroban_inefficient_bytes_concat` lint
//! correctly triggers when `Bytes` container mutation methods such as `push_back` or
//! `append` are invoked repeatedly inside unbounded or large loop constructs.
//!
//! ## Contributor & Maintainer Notes
//! - The mock `soroban_sdk` module simulates the minimal Soroban SDK `Bytes` container handle
//!   necessary for compilation during standalone UI test runs without requiring the full heavy
//!   Soroban SDK dependency tree.
//! - Expected lint diagnostics are explicitly asserted using compiler comment directives
//!   (e.g., `//~ WARNING inefficient Bytes concatenation inside a loop`).
//! - Ensure that any modifications to the lint diagnostic messages are mirrored accurately here.

#![warn(soroban_inefficient_bytes_concat)]

/// Mock implementation of the Soroban SDK components required for testing byte container lints.
pub mod soroban_sdk {
    /// Represents a Soroban SDK `Bytes` heap-allocated byte container.
    pub struct Bytes;

    impl Bytes {
        /// Appends a single byte or value to the back of the `Bytes` container.
        /// Repeated calls inside a loop incur heavy host interaction overhead.
        pub fn push_back(&mut self, _val: u32) {}

        /// Appends another `Bytes` instance to this container.
        pub fn append(&mut self, _other: &Bytes) {}
    }
}

use soroban_sdk::Bytes;

/// # Inefficient Bytes Concatenation Analysis
///
/// This module contains test scenarios demonstrating how `Bytes` container mutations
/// like `push_back` and `append` inside loops generate excessive host calls and memory overhead.
/// Developers should accumulate bytes in local Rust buffers (`Vec<u8>`) instead and perform
/// a single conversion once outside the loop.

/// Triggers a lint warning because `push_back` is repeatedly called inside an unbounded or large loop,
/// leading to heavy host interaction overhead, frequent heap reallocations, and unnecessary memory churn.
fn bad_push_back(mut b: Bytes) {
    for _ in 0..10 {
        b.push_back(1); //~ WARNING inefficient Bytes concatenation inside a loop
    }
}

/// Demonstrates how to suppress the lint warning when a loop is known to be extremely small
/// and provably bounded, making the overhead negligible.
#[allow(soroban_inefficient_bytes_concat)]
fn good_small_push_back(mut b: Bytes) {
    // False positive mitigation: loop iteration count is very small and statically bounded,
    // so the performance impact is negligible. The `#[allow(...)]` attribute silences the diagnostic.
    for _ in 0..2 {
        b.push_back(1);
    }
}

/// Entry point for the UI test executable.
fn main() {}
