//! UI test fixture for the `soroban_inefficient_bytes_concat` lint.
//!
//! This module tests both positive cases (triggering warnings when `Bytes` methods
//! like `push_back` are invoked repeatedly inside loops) and negative/allowed
//! cases (such as small, bounded loops where the performance impact is negligible).

#![warn(soroban_inefficient_bytes_concat)]

/// Mock implementation of the Soroban SDK types used for static analysis testing.
/// This module mirrors the interface of `soroban_sdk::Bytes` to enable robust UI testing
/// without requiring a full compiled Soroban SDK dependency in the linter test suite.
pub mod soroban_sdk {
    pub struct Bytes;
    impl Bytes {
        /// Appends a single 32-bit integer to the end of the `Bytes` container.
        pub fn push_back(&mut self, _val: u32) {}
        /// Appends another `Bytes` container to this one.
        pub fn append(&mut self, _other: &Bytes) {}
    }
}
use soroban_sdk::Bytes;

/// Positive test case: repeatedly pushing back to a `Bytes` container inside a loop.
///
/// # Detailed Mechanics
/// When `push_back` or `append` is called inside a hot loop, each iteration triggers
/// a separate guest-to-host transition. The Soroban VM must allocate memory or copy bytes
/// inside the host environment repeatedly, leading to quadratic or highly linear instruction
/// overhead that quickly exhausts transaction CPU and memory budgets.
///
/// This pattern incurs high memory allocation and CPU instruction costs because each
/// `push_back` crosses the host-guest boundary and forces a new allocation/copy inside
/// the Soroban host environment. It triggers the `soroban_inefficient_bytes_concat`
/// lint warning.
///
/// # Recommendation
/// Developers should accumulate bytes in a local Rust `Vec<u8>` or buffer outside
/// the hot loop path and construct the final Soroban `Bytes` object exactly once
/// after the loop via `Bytes::from_slice` to minimize host invocation fees.
fn bad_push_back(mut b: Bytes) {
    // Iterating and pushing elements one by one inside a loop incurs heavy host invocation overhead.
    for _ in 0..10 {
        b.push_back(1); //~ WARNING inefficient Bytes concatenation inside a loop
    }
}

/// Negative/Allowed test case: small, provably bounded loop using `#[allow(...)]`.
///
/// # Suppression Rationale
/// While the loop contains a `Bytes::push_back` method call, the loop bounds are very
/// small and statically fixed (`0..2`), rendering the host-call overhead negligible in
/// practice. However, because static analysis relies on structural patterns rather than
/// evaluating exact iteration counts, this pattern would normally trigger a warning.
///
/// Using `#[allow(soroban_inefficient_bytes_concat)]` explicitly informs the linter
/// that the developer has audited the hot path and determined the performance impact
/// is acceptable for this specific micro-loop, preventing noisy diagnostics.
#[allow(soroban_inefficient_bytes_concat)]
fn good_small_push_back(mut b: Bytes) {
    for _ in 0..2 {
        b.push_back(1);
    }
}

fn main() {}
