//! UI test fixture for `soroban_inefficient_bytes_concat`.
//!
//! This fixture verifies that the static analysis linter correctly detects
//! inefficient byte concatenations or element pushes (such as `.push_back()`
//! or `.append()`) occurring inside loops on Soroban `Bytes` objects, while
//! respecting allowed exemptions or provably small bounded loops where appropriate.

#![warn(soroban_inefficient_bytes_concat)]

/// Mock implementation of the Soroban SDK structures and methods needed
/// to simulate `Bytes` operations for AST and HIR traversal during linting.
pub mod soroban_sdk {
    pub struct Bytes;
    impl Bytes {
        /// Appends a single value to the back of the Bytes container.
        pub fn push_back(&mut self, _val: u32) {}
        /// Appends another Bytes container to this one.
        pub fn append(&mut self, _other: &Bytes) {}
    }
}
use soroban_sdk::Bytes;

/// Positive test case: performing repeated `push_back` operations inside an
/// unconstrained or sufficiently large loop triggers a warning because it
/// leads to quadratic memory reallocations and excessive memory metering costs.
fn bad_push_back(mut b: Bytes) {
    for _ in 0..10 {
        b.push_back(1); //~ WARNING inefficient Bytes concatenation inside a loop
    }
}

/// Negative test case: small, provably bounded loops are explicitly allowed
/// or annotated to bypass the lint when performance overhead is negligible.
#[allow(soroban_inefficient_bytes_concat)]
fn good_small_push_back(mut b: Bytes) {
    // False positive: loop is small and provably bounded, so cost is negligible,
    // but lint flags it anyway unless allowed.
    for _ in 0..2 {
        b.push_back(1);
    }
}

// Main entry point for the inefficient bytes concat UI test
fn main() {}
