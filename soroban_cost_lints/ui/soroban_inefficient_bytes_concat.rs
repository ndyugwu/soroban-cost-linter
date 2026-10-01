#![warn(soroban_inefficient_bytes_concat)]

//! # Soroban Inefficient Bytes Concat UI Test
//!
//! This module provides UI test cases for the `soroban_inefficient_bytes_concat` lint.
//! It simulates the `soroban_sdk::Bytes` type and demonstrates both inefficient
//! append/push_back patterns inside loops (which trigger warnings) and allowed small
//! bounded loops.

pub mod soroban_sdk {
    pub struct Bytes;
    impl Bytes {
        pub fn push_back(&mut self, _val: u32) {}
        pub fn append(&mut self, _other: &Bytes) {}
    }
}
use soroban_sdk::Bytes;

/// Demonstrates an inefficient bytes concatenation pattern inside a loop.
/// This function triggers the `soroban_inefficient_bytes_concat` lint warning.
fn bad_push_back(mut b: Bytes) {
    for _ in 0..10 {
        b.push_back(1); //~ WARNING: soroban_inefficient_bytes_concat
    }
}

#[allow(soroban_inefficient_bytes_concat)]
/// Demonstrates a bounded loop with small iterations where the lint is allowed.
fn good_small_push_back(mut b: Bytes) {
    // False positive: loop is small and provably bounded, so cost is negligible,
    // but lint flags it anyway unless allowed.
    for _ in 0..2 {
        b.push_back(1);
    }
}

/// Entry point for the UI test executable.
fn main() {}
