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
///
/// Repeatedly appending elements via `push_back` or similar growth methods inside a loop
/// leads to exceptionally high CPU instructions and memory reallocation overhead in Soroban.
/// Each individual `push_back` crosses the host VM-to-host boundary, driving up CPU metering
/// instructions and network resource fees unnecessarily.
///
/// To avoid this high cost overhead, accumulation of elements should instead be performed in a native
/// Rust `Vec<u8>` first, and then converted to Soroban `Bytes` once outside the loop via
/// `Bytes::from_slice(&env, &vec)` or equivalent batch construction.
fn bad_push_back(mut b: Bytes) {
    for _ in 0..10 {
        b.push_back(1); //~ WARNING inefficient Bytes concatenation inside a loop
    }
}

/// Demonstrates a small, bounded loop containing `push_back` operations.
/// This function is explicitly marked with the `#[allow(soroban_inefficient_bytes_concat)]`
/// attribute to suppress the warning since the loop bound is tiny, static, and provably bounded,
/// meaning the accumulated host call overhead is negligible in practice and safely bypassed.
#[allow(soroban_inefficient_bytes_concat)]
fn good_small_push_back(mut b: Bytes) {
    // Deliberate small iteration: loop bound is extremely small (0..2),
    // so cost overhead is negligible and safely suppressed.
    for _ in 0..2 {
        b.push_back(1);
    }
}

/// Entry point for the UI test case.
fn main() {}
