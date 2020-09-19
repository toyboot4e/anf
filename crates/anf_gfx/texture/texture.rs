//! [`TextureData2D`] & [`SpriteData`]

use std::{
    fs::File,
    io::{BufReader, Read, Seek},
    os::raw::c_void,
};

/// 2D texture handle
///
/// # Safety
///
/// It's NOT guaranteed that the internal texture is still alive because it's using a pointer.
#[derive(Debug, PartialEq, Clone)]
pub struct TextureData2D {
    raw: *mut fna3d::Texture,
    pub(crate) w: u32,
    pub(crate) h: u32,
    pub(crate) fmt: fna3d::SurfaceFormat,
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

impl TextureData2D {
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

/// Accessors
impl TextureData2D {
    /// Size in `f32`
    ///
    /// `f32` is is reasonable in rendering context
    pub fn size(&self) -> [f32; 2] {
        [self.w as f32, self.h as f32]
    }

    /// Size in pixels
    pub fn size_px(&self) -> [u32; 2] {
        [self.w, self.h]
    }
}

/// Texture loading methods
/// ---
impl TextureData2D {
    pub fn from_path(
        device: &mut impl AsMut<fna3d::Device>,
        path: impl AsRef<std::path::Path>,
    ) -> Option<Self> {
        let path = path.as_ref();
        let reader = File::open(path)
            .ok()
            .unwrap_or_else(|| panic!("failed to open file {}", path.display()));
        let reader = BufReader::new(reader); // FIXME: is this good?
        Self::from_reader(device, reader)
    }

    pub fn from_reader<R: Read + Seek>(
        device: &mut impl AsMut<fna3d::Device>,
        reader: R,
    ) -> Option<Self> {
        let (pixels_ptr, len, [w, h]) = fna3d::img::from_reader(reader, None);

        if pixels_ptr == std::ptr::null_mut() {
            return None;
        }

        let texture = {
            let pixels_slice = unsafe { std::slice::from_raw_parts(pixels_ptr, len as usize) };
            Self::from_pixels(device, pixels_slice, w, h)
        };

        fna3d::img::free(pixels_ptr as *mut _);
        return Some(texture);
    }

    pub fn from_pixels(
        device: &mut impl AsMut<fna3d::Device>,
        pixels: &[u8],
        w: u32,
        h: u32,
    ) -> Self {
        let device = device.as_mut();
        let mut t = Self::with_size(device, w, h);
        t.set_data(device, 0, None, pixels);
        t
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