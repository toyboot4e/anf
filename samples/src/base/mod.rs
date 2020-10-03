//! Framework on top of ANF for sample games
//!
//! Because Rust does not have inheritance, ANF requires user to write a lot of code to build their
//! own basis. User would build their own lifecycle on top of it, maybe specifying situations such
//! as `debug_render`.
//!
//! * [`framework`]: provides context/user data pattern lifecycle
//! * [`context`]: specific context for the sample games
//!
//! [`AnfLifecycle`]: anf::engine::lifecycle::AnfLifecycle

pub mod context;
pub mod framework;
