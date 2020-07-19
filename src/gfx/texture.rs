//! `Texture2D`

use std::{
    io::{Read, Seek},
    os::raw::c_void,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Texture2D {
    raw: *mut fna3d::Texture,
    pub w: u32,
    pub h: u32,
    pub fmt: fna3d::SurfaceFormat,
    pub level_count: u32,
}

// TODO: what's this
fn calc_mip_levels(w: u32, h: u32, depth: u32) -> u32 {
    use std::cmp::max;
    let mut levels = 1;
    let mut size = max(max(w, h), depth);
    while size > 1 {
        size /= 2;
        levels += 1;
    }
    return levels;
}

fn init_format(fmt: fna3d::SurfaceFormat, is_render_target: bool) -> fna3d::SurfaceFormat {
    use fna3d::SurfaceFormat;
    if !is_render_target
        || !matches!(
            fmt,
            SurfaceFormat::Color
                | SurfaceFormat::Rgba1010102
                | SurfaceFormat::Rg32
                | SurfaceFormat::Rgba64
                | SurfaceFormat::Single
                | SurfaceFormat::Vector2
                | SurfaceFormat::Vector4
                | SurfaceFormat::HalfSingle
                | SurfaceFormat::HalfVector2
                | SurfaceFormat::HalfVector4
                | SurfaceFormat::HdrBlendable
        )
    {
        fmt
    } else {
        fna3d::SurfaceFormat::Color
    }
}

impl Texture2D {
    pub fn raw(&self) -> *mut fna3d::Texture {
        self.raw
    }

    /// Anyway initialize the buffer
    pub fn empty() -> Self {
        Self {
            raw: std::ptr::null_mut(),
            w: 0,
            h: 0,
            fmt: fna3d::SurfaceFormat::Color,
            level_count: 0,
        }
    }

    fn from_size(device: &mut fna3d::Device, w: u32, h: u32) -> Self {
        Self::new(device, w, h, false, fna3d::SurfaceFormat::Color)
    }

    // TODO: what is mip map
    fn new(
        device: &mut fna3d::Device,
        w: u32,
        h: u32,
        do_mip_map: bool,
        fmt: fna3d::SurfaceFormat,
    ) -> Self {
        let is_render_target = false; // FIXME:
        let level_count = if do_mip_map {
            self::calc_mip_levels(w, h, 0)
        } else {
            1
        };
        let fmt = self::init_format(fmt, is_render_target);
        let raw = device.create_texture_2d(fmt, w, h, level_count, is_render_target);

        Self {
            raw,
            w,
            h,
            fmt,
            level_count,
        }
    }

    pub fn from_path(
        device: &mut fna3d::Device,
        path: impl AsRef<std::path::Path>,
    ) -> Option<Self> {
        let path = path.as_ref();
        let reader = std::fs::File::open(path)
            .ok()
            .unwrap_or_else(|| panic!("failed to open file {}", path.display()));
        let reader = std::io::BufReader::new(reader);
        Self::from_reader(device, reader)
    }

    pub fn from_reader<R: Read + Seek>(device: &mut fna3d::Device, reader: R) -> Option<Self> {
        let (pixels, len, [w, h]) = fna3d::img::load_image_from_reader(reader, None, false);

        if pixels.is_null() {
            return None;
        }

        log::trace!(
            "load texture: {{ len: {}, w: {}, h: {} }}, pixels at {:?}",
            len,
            w,
            h,
            pixels
        );

        let mut texture = Self::from_size(device, w, h);
        texture.set_data_ptr(device, 0, None, pixels as *mut _, len as u32);

        unsafe {
            fna3d::sys::FNA3D_Image_Free(pixels);
        }

        return Some(texture);
    }

    /// Set texture data from a pointer
    pub fn set_data_ptr(
        &mut self,
        device: &mut fna3d::Device,
        level: u32,
        // TODO: what is this
        rect: Option<[u32; 4]>,
        data: *mut c_void,
        data_len_in_bytes: u32,
    ) {
        let (x, y, w, h) = if let Some(xs) = rect {
            (xs[0], xs[1], xs[2], xs[3])
        } else {
            (
                0,
                0,
                std::cmp::max(self.w >> level, 1),
                std::cmp::max(self.h >> level, 1),
            )
        };

        device.set_texture_data_2d(
            self.raw,
            self.fmt,
            x,
            y,
            w,
            h,
            level,
            data,
            data_len_in_bytes,
        );
    }
}
