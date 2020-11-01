/*! ANF is a 2D framework powered by [SDL] & [FNA3D] ([Rust-SDL2] and [Rust-FNA3D])

Note that ANF is unstable and experimental; ANF does not provide solutions in most areas -- you
have to do it yourself.

# Note

* Right-handed coordinate system
* Row-major matrices

# Restrictions

* One-window, single-threaded
* Pre-defined game loop

FIXME: `cargo test` fails in ANF (so using `no_run` for now)

[SDL]: https://www.sdl.com/
[FNA3D]: https://github.com/FNA-XNA/FNA3D
[Rust-SDL2]: https://github.com/Rust-SDL2/rust-sdl2
[Rust-FNA3D]: https://github/toyboot4e/rust-fna3d
[anf/examples]: https://github/toyboot4e/anf/examples
!*/

pub mod engine;
pub mod gfx;
pub mod prim;
pub mod vfs;
