//! `Texture2D`
//!
//! TODO: explain what are sampler and surface

use std::{
    io::{BufRead, Read, Seek},
    os::raw::c_void,
};

// TODO: add helper for pixel texture with color
// TODO: lifetime

/// Wraps a texture handle with some metadata and `SurfaceFormat`
///
/// * `level_count:`
///   TODO: what is this
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

fn get_init_format(fmt: fna3d::SurfaceFormat, is_render_target: bool) -> fna3d::SurfaceFormat {
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
            level_count: 0,
        }
    }

    pub fn with_size(device: &mut fna3d::Device, w: u32, h: u32) -> Self {
        Self::new(device, w, h, false, fna3d::SurfaceFormat::Color)
    }

    #[inline]
    pub fn new(
        device: &mut fna3d::Device,
        w: u32,
        h: u32,
        do_mip_map: bool,
        fmt: fna3d::SurfaceFormat,
    ) -> Self {
        Self::new_impl(device, w, h, do_mip_map, fmt, false)
    }

    /// Creates a `Texture2D` as a render target
    #[inline]
    fn new_target(
        device: &mut fna3d::Device,
        w: u32,
        h: u32,
        do_mip_map: bool,
        fmt: fna3d::SurfaceFormat,
    ) -> Self {
        Self::new_impl(device, w, h, do_mip_map, fmt, true)
    }

    fn new_impl(
        device: &mut fna3d::Device,
        w: u32,
        h: u32,
        do_mip_map: bool,
        fmt: fna3d::SurfaceFormat,
        is_render_target: bool,
    ) -> Self {
        let level_count = if do_mip_map {
            self::calc_mip_levels(w, h, 0)
        } else {
            1
        };
        let fmt = self::get_init_format(fmt, is_render_target);
        let raw = device.create_texture_2d(fmt, w, h, level_count, is_render_target);

        Self {
            raw,
            w,
            h,
            fmt,
            level_count,
        }
    }
}

impl Texture2D {
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

    // pub fn from_reader<R: BufRead + Seek>(
    pub fn from_reader<R: Read + Seek>(device: &mut fna3d::Device, reader: R) -> Option<Self> {
        // this is broken
        // use sdl2::image::ImageRWops;
        // let mut buf = Vec::new();
        // let x = sdl2::rwops::RWops::from_read(&mut reader, &mut buf).unwrap();
        // let sur = x.load().unwrap();
        // let w = sur.width();
        // let h = sur.height();
        // let len = sur.pitch();
        // let pixels = unsafe { (*sur.raw()).pixels };

        // is this broken?
        // TODO: try loading image using something else
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

        let mut texture = Self::with_size(device, w, h);
        texture.set_data_ptr(device, 0, None, pixels as *mut _, len as u32);

        unsafe {
            fna3d::sys::FNA3D_Image_Free(pixels);
        }

        return Some(texture);
    }

    /// Sets texture data from a slice
    pub fn set_data<T>(
        &mut self,
        device: &mut fna3d::Device,
        level: u32,
        // TODO: what is this
        rect: Option<[u32; 4]>,
        data: &[T],
    ) {
        self.set_data_ptr(
            device,
            level,
            rect,
            data as *const _ as *mut _,
            (std::mem::size_of::<T>() * data.len()) as u32,
        )
    }

    /// Sets texture data from a pointer
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

// --------------------------------------------------------------------------------
// TODO: RenderTarget2D

// TODO: add RenderTargetBindings to pipeline

/// Wrapper around `Texture2D` as a render target (a.k.a. canvas)
pub struct RenderTarget2D {
    tx: Texture2D,
    ds_fmt: fna3d::DepthFormat,
    multi_sample_count: u32,
    usage: fna3d::RenderTargetUsage,
    is_content_lost: bool,
    gl_depth_stencil_buffer: *mut fna3d::Renderbuffer,
    gl_color_buffer: *mut fna3d::Renderbuffer,
}

impl RenderTarget2D {
    pub fn new(
        device: &mut fna3d::Device,
        w: u32,
        h: u32,
        do_mip_map: bool,
        surface: fna3d::SurfaceFormat,
        ds_fmt: fna3d::DepthFormat,
        preferred_multi_sample_count: u32,
        usage: fna3d::RenderTargetUsage,
    ) -> Self {
        let tx = Texture2D::new_target(device, w, h, do_mip_map, surface);

        let multi_sample_count = device.get_max_multi_sample_count(
            surface,
            self::closest_msaa_power(preferred_multi_sample_count) as i32,
        );

        let mut me = Self {
            tx,
            ds_fmt,
            multi_sample_count: multi_sample_count as u32,
            usage,
            is_content_lost: false,
            gl_depth_stencil_buffer: std::ptr::null_mut(),
            gl_color_buffer: std::ptr::null_mut(),
        };

        if multi_sample_count > 0 {
            me.gl_color_buffer =
                device.gen_color_renderbuffer(w, h, surface, multi_sample_count, me.tx.raw());
        }

        if ds_fmt != fna3d::DepthFormat::None {
            me.gl_depth_stencil_buffer =
                device.gen_depth_stencil_renderbuffer(w, h, ds_fmt, multi_sample_count);
        }

        me
    }
}

fn closest_msaa_power(value: u32) -> u32 {
    /* Checking for the highest power of two _after_ than the given int:
     * http://graphics.stanford.edu/~seander/bithacks.html#RoundUpPowerOf2
     * Take result, divide by 2, get the highest power of two _before_!
     */
    if value == 1 {
        // ... Except for 1, which is invalid for MSAA -flibit
        return 0;
    }
    let mut result: u32 = value - 1;
    result |= result >> 1;
    result |= result >> 2;
    result |= result >> 4;
    result |= result >> 8;
    result |= result >> 16;
    result += 1;
    if result == value {
        result
    } else {
        result >> 1
    }
}

// pub fn gen_rect_texture(device: &mut fna3d::Device, color: fna3d::Color) -> Texture2D {
//     // TODO: what is level count
//     let mut t = RenderTarget2D::new(device, 50, 50, false, fna3d::SurfaceFormat::Color);
//     device.set_render_targets(
//         render_targets,
//         num_render_targets,
//         depth_stencil_buffer,
//         depth_format,
//     );
//     t
// }

// fn set_rt(
//     device: &mut fna3d::Device,
//     params: &fna3d::PresentationParameters,
//     rt: &mut RenderTarget2D, // Option
// ) {
//     device.set_render_targets(None, 0, None, fna3d::DepthFormat::None);

//     // Set the viewport/scissor to the size of the backbuffer.
//     let new_w = params.backBufferWidth;
//     let new_h = params.backBufferHeight;
//     use fna3d::enum_primitive::*;
//     let clear_target = fna3d::RenderTargetUsage::from_u32(params.renderTargetUsage).unwrap();

//     // Resolve previous targets, if needed
//     // device.resolve_target(&nativeTargetBindings);
//     // Array.Clear(renderTargetBindings, 0, renderTargetBindings.Length);
//     // Array.Clear(nativeTargetBindings, 0, nativeTargetBindings.Length);
//     // renderTargetCount = 0;

//     // Apply new GL state, clear target if requested
//     let vp = fna3d::Viewport {
//         x: 0,
//         y: 0,
//         w: new_w,
//         h: new_h,
//         minDepth: 0 as f32,
//         maxDepth: 0 as f32,
//     };
//     device.set_viewport(&vp);

//     let scissor = fna3d::Rect {
//         x: 0,
//         y: 0,
//         w: new_w,
//         h: new_h,
//     };
//     device.set_scissor_rect(Some(scissor));

//     if clear_target == fna3d::RenderTargetUsage::DiscardContents {
//         clear(
//             device,
//             fna3d::ClearOptions::Target
//                 | fna3d::ClearOptions::DepthBuffer
//                 | fna3d::ClearOptions::Stencil,
//             fna3d::colors::rgb(0, 0, 0),
//             vp.maxDepth,
//             0,
//         );
//     }
// }

// fn clear(
//     device: &mut fna3d::Device,
//     opts: fna3d::ClearOptions,
//     color: fna3d::Color,
//     depth: f32,
//     stencil: u32,
// ) {
//     //
// }
