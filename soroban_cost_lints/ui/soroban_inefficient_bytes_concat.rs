//! UI test fixture for the `soroban_inefficient_bytes_concat` lint.
//! 
//! This module tests detection of inefficient byte/vector concatenations 
//! (such as repeated `.push_back()` or `.append()` calls) inside unbounded or 
//! large loops, which unnecessarily drive up Soroban resource metering and 
//! memory allocation costs.

#![warn(soroban_inefficient_bytes_concat)]

/// Mock Soroban SDK definitions required for testing the lint diagnostic output.
pub mod soroban_sdk {
    /// Represents a Soroban SDK `Bytes` container handle.
    pub struct Bytes;
    impl Bytes {
        /// Appends a single value to the back of the byte container.
        pub fn push_back(&mut self, _val: u32) {}
        
        /// Appends another byte slice or container.
        pub fn append(&mut self, _other: &Bytes) {}
    }
}
use soroban_sdk::Bytes;

/// Triggers the `soroban_inefficient_bytes_concat` warning because elements
/// are repeatedly pushed to a `Bytes` container inside a loop of size 10.
fn bad_push_back(mut b: Bytes) {
    for _ in 0..10 {
        b.push_back(1); //~ WARNING inefficient Bytes concatenation inside a loop
    }
}

/// Does not trigger the lint due to an explicit `#[allow(...)]` attribute,
/// demonstrating how to suppress false positives on provably small/bounded loops
/// where the allocation overhead is negligible.
#[allow(soroban_inefficient_bytes_concat)]
fn good_small_push_back(mut b: Bytes) {
    // False positive: loop is small and provably bounded, so cost is negligible,
    // but lint flags it anyway unless allowed.
    for _ in 0..2 {
        b.push_back(1);
    }
}

fn main() {}
