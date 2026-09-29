//! # Soroban Inefficient Bytes Concat Lint UI Test
//!
//! This UI test file validates the diagnostic output and suggestions generated
//! by the `soroban_inefficient_bytes_concat` lint.
//!
//! ## Overview
//! Soroban smart contracts often need to concatenate `Bytes` or `BytesN` values.
//! Using repeated append or push_back operations inside loops leads to
//! excessive CPU and memory host-call costs within Soroban's metering model.
//! This lint detects such inefficient patterns and suggests optimized alternatives.

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
pub mod soroban_sdk {
    pub struct Env;
    impl Env {
        pub fn default() -> Self {
            Env
        }
    }

    pub struct Bytes;
    impl Bytes {
        pub fn new(_env: &Env) -> Self {
            Bytes
        }
        pub fn push_back(&mut self, _val: u32) {}
        pub fn append(&mut self, _other: &Bytes) {}
    }
}

use soroban_sdk::Bytes;

fn bad_push_back(mut b: Bytes) {
    for _ in 0..10 {
        b.push_back(1); //~ ERROR inefficient Bytes concatenation inside a loop
    }
}

#[allow(soroban_inefficient_bytes_concat)]
fn good_small_push_back(mut b: Bytes) {
    for _ in 0..2 {
        b.push_back(1);
    }
}

fn main() {
    let b = Bytes::new(&soroban_sdk::Env::default());
    bad_push_back(b);
}
