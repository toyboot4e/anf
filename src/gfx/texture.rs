//! 2D texture

use std::os::raw::c_void;

/// A 2D texture handle with some metadata
///
/// * TODO: `Rc`? lifetime?
///
/// # Lacking features
///
/// * mipmap
#[derive(Debug, PartialEq, Clone)]
pub struct Texture2D {
    raw: *mut fna3d::Texture,
    pub w: u32,
    pub h: u32,
    pub fmt: fna3d::SurfaceFormat,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureKind {
    Texture,
    RenderTarget,
}

fn get_init_format(fmt: fna3d::SurfaceFormat, kind: TextureKind) -> fna3d::SurfaceFormat {
    let is_render_target = kind == TextureKind::RenderTarget;

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
        fna3d::SurfaceFormat::Color
    } else {
        fmt
    }
}

impl Texture2D {
    pub fn raw(&self) -> *mut fna3d::Texture {
        self.raw
    }

    pub fn empty() -> Self {
        Self {
            raw: std::ptr::null_mut(),
            w: 0,
            h: 0,
            fmt: fna3d::SurfaceFormat::Color,
        }
    }

    pub fn new(
        device: &mut fna3d::Device,
        w: u32,
        h: u32,
        fmt: fna3d::SurfaceFormat,
        kind: TextureKind,
    ) -> Self {
        let level = 1;
        let fmt = self::get_init_format(fmt, TextureKind::Texture);
        let raw = device.create_texture_2d(fmt, w, h, level, kind == TextureKind::RenderTarget);

        Self { raw, w, h, fmt }
    }

    pub fn with_size(device: &mut fna3d::Device, w: u32, h: u32) -> Self {
        Self::new(
            device,
            w,
            h,
            fna3d::SurfaceFormat::Color,
            TextureKind::Texture,
        )
    }
}

/// Texture loading methods
/// ---
impl Texture2D {
    pub fn from_path(
        device: &mut fna3d::Device,
        path: impl AsRef<std::path::Path>,
    ) -> Option<Self> {
        let (pixels_ptr, len, [w, h]) = fna3d::img::from_path(path, None);
        if pixels_ptr == std::ptr::null_mut() {
            return None;
        }

        let texture = {
            let mut t = Self::with_size(device, w, h);
            let pixels_slice = unsafe { std::slice::from_raw_parts(pixels_ptr, len as usize) };
            t.set_data(device, 0, None, pixels_slice);
            t
        };

        fna3d::img::free(pixels_ptr as *mut _);

        return Some(texture);
    }

    /// Sets texture data on GPU memory
    pub fn set_data(
        &mut self,
        device: &mut fna3d::Device,
        level: u32,
        rect: Option<[u32; 4]>,
        data: &[u8],
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
            x,
            y,
            w,
            h,
            level,
            data as *const [u8] as *mut c_void,
            data.len() as u32,
        );
    }
}
