# Thread Local Panic Hook

A simple crate that implements a `std::panic::{set_hook, take_hook, update_hook}` drop
in replacements that work per thread.

When target family is `wasm`, this crate does nothing and just re-exports the std versions.
