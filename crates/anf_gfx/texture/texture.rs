//! [`TextureData2d`]

use std::{
    fs::File,
    io::{BufReader, Read, Seek},
    rc::Rc,
};

use fna3h::{tex::Texture, Device, SurfaceFormat};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureKind {
    Texture,
    RenderTarget,
}

/// 2D GPU texture handle
///
/// Automatically disposes the FNA3D texture when dropping.
#[derive(Debug, Clone)]
pub struct Texture2dDrop {
    raw: *mut Texture,
    device: Device,
    pub(crate) w: u32,
    pub(crate) h: u32,
    pub(crate) fmt: SurfaceFormat,
}

unsafe impl Send for Texture2dDrop {}
unsafe impl Sync for Texture2dDrop {}

impl PartialEq<Self> for Texture2dDrop {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw && self.w == other.w && self.h == other.h && self.fmt == other.fmt
    }
}

impl Drop for Texture2dDrop {
    fn drop(&mut self) {
        self.device.add_dispose_texture(self.raw);
    }
}

impl Texture2dDrop {
    pub fn new(device: &Device, w: u32, h: u32, fmt: SurfaceFormat, kind: TextureKind) -> Self {
        let fmt = self::get_init_format(fmt, TextureKind::Texture);
        let raw = device.create_texture_2d(fmt, w, h, 1, kind == TextureKind::RenderTarget);

        Texture2dDrop {
            device: device.clone(),
            raw,
            w,
            h,
            fmt,
        }
    }

    pub fn with_size(device: &Device, w: u32, h: u32) -> Self {
        Self::new(device, w, h, SurfaceFormat::Color, TextureKind::Texture)
    }

    pub fn from_path(device: &Device, path: impl AsRef<std::path::Path>) -> Option<Self> {
        let path = path.as_ref();

        // TODO: return error
        let reader = File::open(path).unwrap_or_else(|err| {
            panic!(
                "failed to open file `{}`. io error: {}",
                path.display(),
                err
            )
        });
        let reader = BufReader::new(reader); // FIXME: is this good?

        Self::from_reader(device, reader)
    }

    /// Helper for embedded file bytes
    pub fn from_encoded_bytes(device: &Device, bytes: &[u8]) -> Option<Self> {
        Self::from_reader(device, std::io::Cursor::new(bytes))
    }

    pub fn from_reader<R: Read + Seek>(device: &Device, reader: R) -> Option<Self> {
        let (pixels_ptr, len, [w, h]) = fna3h::img::from_reader(reader, None);

        if pixels_ptr == std::ptr::null_mut() {
            return None;
        }

        let gpu_texture = {
            let pixels_slice = unsafe { std::slice::from_raw_parts(pixels_ptr, len as usize) };
            Self::from_decoded_bytes(device, pixels_slice, w, h)
        };

        fna3h::img::free(pixels_ptr as *mut _);

        return Some(gpu_texture);
    }

    pub fn from_decoded_bytes(device: &Device, pixels: &[u8], w: u32, h: u32) -> Self {
        let t = Self::with_size(device, w, h);
        t.upload_pixels(device, 0, None, pixels);
        t
    }
}

/// Accessors
impl Texture2dDrop {
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

    /// Upload pixels to the GPU (VRAM?) texture data
    pub fn upload_pixels(
        &self,
        device: &Device,
        target_mipmap_level: u32,
        rect: Option<[u32; 4]>,
        data: &[u8],
    ) {
        let (x, y, w, h) = if let Some(xs) = rect {
            (xs[0], xs[1], xs[2], xs[3])
        } else {
            (
                0,
                0,
                std::cmp::max(self.w >> target_mipmap_level, 1),
                std::cmp::max(self.h >> target_mipmap_level, 1),
            )
        };

        device.set_texture_data_2d(self.raw, x, y, w, h, target_mipmap_level, data);
    }

    // /// VERY HEAVY task
    // pub fn save_to_png(&self, path: &Path) {
    //     let pixels = Vec::with_capacity(32 * self.w as usize * self.h as usize);
    //     self.device
    //         .get_texture_data_2d(self.raw, self.x, self.y, self.w, self.h, 0, &mut pixels);
    //     // TODO:
    // }
}

fn get_init_format(fmt: SurfaceFormat, kind: TextureKind) -> SurfaceFormat {
    let is_render_target = kind == TextureKind::RenderTarget;

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
        SurfaceFormat::Color
    } else {
        fmt
    }
}

/// Reference counted 2D GPU texture
#[derive(Debug, PartialEq, Clone)]
pub struct TextureData2d {
    inner: Rc<Texture2dDrop>,
}

impl std::ops::Deref for TextureData2d {
    type Target = Texture2dDrop;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TextureData2d {
    pub fn raw(&self) -> *mut Texture {
        self.raw
    }

    pub fn new(device: &Device, w: u32, h: u32, fmt: SurfaceFormat, kind: TextureKind) -> Self {
        Self {
            inner: Rc::new(Texture2dDrop::new(device, w, h, fmt, kind)),
        }
    }

    pub fn with_size(device: &Device, w: u32, h: u32) -> Self {
        Self::new(device, w, h, SurfaceFormat::Color, TextureKind::Texture)
    }
}

/// Texture loading methods
/// ---
impl TextureData2d {
    pub fn from_drop(d: Texture2dDrop) -> Self {
        Self { inner: Rc::new(d) }
    }

    pub fn from_path(device: &Device, path: impl AsRef<std::path::Path>) -> Option<Self> {
        Some(Self::from_drop(Texture2dDrop::from_path(device, path)?))
    }

    /// Helper for embedded file bytes
    pub fn from_encoded_bytes(device: &Device, bytes: &[u8]) -> Option<Self> {
        Some(Self::from_drop(Texture2dDrop::from_encoded_bytes(
            device, bytes,
        )?))
    }

    pub fn from_reader<R: Read + Seek>(device: &Device, reader: R) -> Option<Self> {
        Some(Self::from_drop(Texture2dDrop::from_reader(device, reader)?))
    }

    pub fn from_decoded_bytes(device: &Device, pixels: &[u8], w: u32, h: u32) -> Self {
        Self::from_drop(Texture2dDrop::from_decoded_bytes(device, pixels, w, h))
    }
}
