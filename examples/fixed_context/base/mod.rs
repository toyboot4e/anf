//! Base of the sample games
//!
//! Because Rust does not have inheritance, ANF requires user to write a lot of code to build their
//! own basis. User would build their own lifecycle on top of it, maybe specifying situations such
//! as `debug_render`.
//!
//! * [`framework`]: provides context/user data pattern lifecycle
//! * [`context`]: specific context for the sample games
//! * [`scene`]: specific user data for the sample games
//!
//! [`AnfLifecycle`]: anf::engine::lifecycle::AnfLifecycle

// context/user-data pattern
pub mod framework;

// context for the sample games
pub mod context;

// user data for the sample games
pub mod scene;
