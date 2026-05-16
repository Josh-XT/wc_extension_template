//! Local validation harness for the example WorkConductor extension.
//!
//! `src/example_extension.rs` is the file intended to be copied into
//! `WorkConductor/agixt-rust/crates/agixt-extensions/src/`. This crate re-exports
//! WorkConductor's extension traits at `crate::traits` so that same file compiles
//! both here and inside WorkConductor.

pub mod traits {
    pub use agixt_extensions::traits::*;
}

pub mod example_extension;

pub use example_extension::{ExampleExtension, ExampleItem};
