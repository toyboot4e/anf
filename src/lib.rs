//! ANF is a 2D framework powered by FNA3D
//!
//! See the [`app`] module or [examples] to get started.
//!
//! ## Meta
//!
//! ANF is for [my roguelike game development] and it's not going to be a feature-complete 2D
//! framework. However, you only need 5 minutes to read this API documentation to get the picture,
//! so you _could_ like the "simplicity"!
//!
//! [audio]: https://lib.rs/multimedia/audio
//! [`app`]: ./app.index.html
//! [examples]: https://github/toyboot4e/anf/examples
//! [my roguelike game development]: https://github/toyboot4e/rlbox

// `Color` and other types
pub use fna3d;
// pub use sdl2;

pub mod app;
pub mod gfx;
pub mod vfs;

pub mod _todos_ {
    //! Notes
    //!
    //! ## Infructure
    //!
    //! * copy FNA3D to output
    //! * copy `assets/` to output
    //!
    //! ## API
    //!
    //! * hide `fna3d` (e.g. re-export `fna3d::Color` to `anf::gfx`)
    //!
    //! ## Features
    //!
    //! * viewport etc.
    //! * `Texture2D` with or without lifetime
    //! * content loader (cache `Teture2D`)
    //! * async texture loading
    //!
    //! ## impls
    //!
    //! * free memory on neessary
    //! * FPS
    //!
    //! ## Improve Rust-FNA3D
    //!
    //! * use render target bindings
}
