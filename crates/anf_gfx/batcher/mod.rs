//! Internals of quad rendering
//!
//! * TODO: flush on push if it's out of capacity

pub mod batch;
pub mod bufspecs;
pub mod primitives;

mod batcher;
pub use batcher::Batcher;
