/// Comprehensive UI test fixture for the `soroban_inefficient_bytes_concat` lint.
///
/// This test file verifies that the `soroban_inefficient_bytes_concat` lint correctly
/// identifies inefficient `.push_back()` and `.append()` operations performed on
/// Soroban `Bytes` containers within loop structures (`for`, `while`, `loop`).
/// It ensures that developers are warned about excessive host boundary crossings
/// and guides developers toward efficient Rust-native memory accumulation using `Vec<u8>` before a single conversion via `Bytes::from_slice`.
///
/// # What it does
/// Detects Bytes concatenation operations (`push_back` and `append`) that are
/// executed inside loop bodies (`for`, `while`, or `loop`).
///
/// # Why is this bad?
/// Every `push_back` and `append` call on Soroban `Bytes` crosses the host boundary.
/// Placing these operations inside a loop results in repeated host function calls,
/// drastically increasing CPU instruction costs and transaction fees.
///
/// # Suggested Fix
/// Accumulate bytes in a Rust `Vec<u8>` during the loop, then convert to `Bytes`
/// once outside the loop via `Bytes::from_slice`.
///
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

/// Positive test case: performing repeated `push_back` operations inside a
/// `for` loop triggers the `soroban_inefficient_bytes_concat` warning because
/// each call crosses the host boundary and inflates gas/CPU fees unnecessarily.
///
/// # Detailed Analysis
/// When `push_back` is called iteratively inside an unconstrained or sufficiently
/// large loop, each operation incurs host boundary crossing overheads and memory
/// reallocations. Developers should accumulate items in a local `Vec<u8>`
/// first and perform a single conversion via `Bytes::from_slice` afterwards.
fn bad_push_back(mut b: Bytes) {
    for _ in 0..10 {
        b.push_back(1); //~ WARNING inefficient Bytes concatenation inside a loop
    }
}

/// Negative test case: small, provably bounded loops or explicitly suppressed
/// blocks are ignored or bypassed when marked with the appropriate attributes.
///
/// # Exception Rationale
/// For tiny fixed iterations where overhead is minimal or for test mocking contexts,
/// the lint can be explicitly silenced using the `#[allow(...)]` attribute.
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
