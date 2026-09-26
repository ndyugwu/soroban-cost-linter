#![warn(soroban_inefficient_bytes_concat)]

//! UI test case for `soroban_inefficient_bytes_concat` lint.
//! This file demonstrates both inefficient pattern usage and allowed false positives.

pub mod soroban_sdk {
    pub struct Bytes;
    impl Bytes {
        pub fn push_back(&mut self, _val: u32) {}
        pub fn append(&mut self, _other: &Bytes) {}
    }
}
use soroban_sdk::Bytes;

/// Demonstrates an inefficient bytes concatenation pattern inside a loop
/// which triggers the `soroban_inefficient_bytes_concat` lint warning.
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

/// Entry point for the UI test case.
fn main() {}
