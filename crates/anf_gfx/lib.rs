/*!

Internals of ANF graphics built on top of [`fna3d`]

# Row-major

ANF thinks **position vectors are row vectors**. So most matrices in mathmatical textbooks are
transposed.

* TODO: Mesh { vbuf, ibuf }
* TODO: Material { bst,  sst, dst, rst, }

*/

pub mod batcher;
pub mod cmd;
pub mod geom2d;
pub mod geom3d;
pub mod texture;
