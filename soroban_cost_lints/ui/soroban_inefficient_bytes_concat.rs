use soroban_sdk::Bytes;
//! # Inefficient Bytes Concat Lint UI Test
//!
//! This UI test file validates the diagnostic output and suggestions generated
//! by the `soroban_inefficient_bytes_concat` lint.
//!
//! ## Overview
//! Soroban smart contracts often need to concatenate `Bytes` or `BytesN` values.
//! Using repeated append operations or inefficient concatenation patterns can lead to
//! excessive CPU and memory CPU-cost charges within Soroban's metering model.
//! This lint detects such inefficient patterns and suggests optimized alternatives.
//!
//! ## Usage in UI Testing
//! This file serves as a negative/positive test case processed by `trybuild` or
//! similar UI test harnesses to ensure compiler diagnostics match expected outputs.


pub mod soroban_sdk {
    pub struct Bytes;
    impl Bytes {
        pub fn push_back(&mut self, _val: u32) {}
        pub fn append(&mut self, _other: &Bytes) {}
    }
}

fn bad_push_back(mut b: Bytes) {
    for _ in 0..10 {
        b.push_back(1); //~ WARNING inefficient Bytes concatenation inside a loop
    }
}

#[allow(soroban_inefficient_bytes_concat)]
fn good_small_push_back(mut b: Bytes) {
    // False positive: loop is small and provably bounded, so cost is negligible,
    // but lint flags it anyway unless allowed.
    for _ in 0..2 {
        b.push_back(1);
    }
}
/// Demonstrates an inefficient bytes concatenation pattern that triggers
/// the `soroban_inefficient_bytes_concat` lint.
///
/// # Panics
/// Does not panic; purely structured for lint verification.
fn inefficient_concat() {
    // Constructing or concatenating bytes in a suboptimal way
    let mut b = soroban_sdk::Bytes::new(&soroban_sdk::Env::default());
    b.push(1);
    b.push(2);
}

/// Entry point for the UI test executable.
fn main() {
    inefficient_concat();
}
