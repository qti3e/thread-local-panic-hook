//! This crate provides a wrapper around `std::panic::{set_hook, take_hook, update_hook}`
//! that work per thread.
//!
//! When building for a wasm, we just re-export the original methods found in the std library.

#[cfg(not(target_family = "wasm"))]
mod panic;

#[cfg(target_family = "wasm")]
mod panic {
    pub use std::panic::{set_hook, take_hook, update_hook};
}

pub use panic::*;
