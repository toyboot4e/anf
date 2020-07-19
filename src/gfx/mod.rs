//! Graphics

pub mod batch;
pub mod prelude;
pub mod texture;
pub mod vertices;

use batch::Batcher;

pub fn begin_frame(device: &mut fna3d::Device) {
    device.begin_frame();
}

/// Makes sure the `Batcher` flushes and swaps buffers
pub fn end_frame(device: &mut fna3d::Device, batcher: &mut Batcher) {
    batcher.flush(device);
    device.swap_buffers(None, None, batcher.win);
}

pub fn clear(device: &mut fna3d::Device) {
    let color = fna3d::Color {
        r: 100,
        g: 149,
        b: 237,
        a: 0,
    };
    device.clear(fna3d::ClearOptions::Target, color, 0.0, 0);
}
