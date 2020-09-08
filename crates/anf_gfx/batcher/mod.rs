//! Internals of quad rendering
//!
//! * TODO: flush on push if it's out of capacity

pub mod batch;
pub mod bufspecs;

mod batcher;
pub use batcher::Batcher;
