//! Font book on font stash

pub use ::fontstash;

use crate::gfx::Color;

pub struct FontBook {
    /// Keeps memory position of the renderer
    inner: Box<FontBookInternal>,
}

impl std::ops::Deref for FontBook {
    type Target = FontBookInternal;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for FontBook {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl FontBook {
    pub fn new(device: fna3d::Device, w: u32, h: u32) -> Self {
        let mut inner = Box::new(FontBookInternal {
            device,
            stash: fontstash::FonsContext::placeholder_null(),
            texture: std::ptr::null_mut(),
            w,
            h,
        });

        inner.stash = fontstash::FonsContext::create(
            w,
            h,
            inner.as_ref() as *const _ as *mut FontBookInternal,
        );

        FontBook { inner }
    }
}

/// The internals of [`FontBook`]
///
/// The memory location should be fixed.
pub struct FontBookInternal {
    device: fna3d::Device,
    stash: fontstash::FonsContext,
    texture: *mut fna3d::Texture,
    w: u32,
    h: u32,
}

impl Drop for FontBookInternal {
    fn drop(&mut self) {
        log::info!("dropping font book");
        // FIXME: why this is not needed
        // fontstash::delete(self.inner.stash.raw());
    }
}

impl FontBookInternal {
    pub fn texture(&self) -> *mut fna3d::Texture {
        self.texture
    }

    pub fn stash(&self) -> fontstash::FonsContext {
        self.stash.clone()
    }

    pub fn text_iter(&mut self, text: &str) -> fontstash::FonsTextIter {
        let iter = self.stash.text_iter(text);
        self.update_texture();
        iter
    }

    /// Updates GPU texure. Call it whenever drawing text
    fn update_texture(&mut self) {
        self.stash.with_pixels(|pixels, w, h| {
            let data = {
                let area = (w * h) as usize;
                // four channels (RGBA)
                let mut data = Vec::<u8>::with_capacity(4 * area);
                for i in 0..area {
                    data.push(255);
                    data.push(255);
                    data.push(255);
                    data.push(pixels[i]);
                }
                data
            };

            self.device
                .set_texture_data_2d(self.texture, 0, 0, w, h, 0, &data);
        });
    }
}

unsafe impl fontstash::Renderer for FontBookInternal {
    /// Return `1` to represent success
    unsafe extern "C" fn create(
        uptr: *mut std::os::raw::c_void,
        width: std::os::raw::c_int,
        height: std::os::raw::c_int,
    ) -> std::os::raw::c_int {
        log::info!("fontstash: create");
        let me = &mut *(uptr as *const _ as *mut Self);

        if !me.texture.is_null() {
            me.device.add_dispose_texture(me.texture);
        }

        fontstash::set_error_callback(me.stash.raw(), fons_error_callback, uptr);

        me.texture = me.device.create_texture_2d(
            fna3d::SurfaceFormat::Color,
            width as u32,
            height as u32,
            0,
            false,
        );

        return 1;

        unsafe extern "C" fn fons_error_callback(
            _uptr: *mut ::std::os::raw::c_void,
            error: ::std::os::raw::c_int,
            _val: ::std::os::raw::c_int,
        ) {
            if error < 0 {
                log::warn!("fons error callback called!: Somehow THE ERROR CODE IS BROKEN");
                return;
            }
            match fontstash::ErrorCode::from_u32(error as u32) {
                Some(error) => {
                    log::warn!("fons error callback called!: {:?}", error);
                }
                None => {
                    log::warn!("fons error callback called!: Somehow THE ERROR CODE IS BROKEN");
                }
            }
        }
    }

    /// Return `1` to represent success. Recreation can be sufficient
    unsafe extern "C" fn resize(
        uptr: *mut std::os::raw::c_void,
        width: std::os::raw::c_int,
        height: std::os::raw::c_int,
    ) -> std::os::raw::c_int {
        log::info!("fontstash: resize");
        let mut me = &mut *(uptr as *const _ as *mut Self);

        1
    }

    unsafe extern "C" fn update(
        uptr: *mut std::os::raw::c_void,
        rect: *mut std::os::raw::c_int,
        data: *const std::os::raw::c_uchar,
    ) {
        log::info!("fontstash: update");
        let mut me = &mut *(uptr as *const _ as *mut Self);
    }

    unsafe extern "C" fn draw(
        uptr: *mut std::os::raw::c_void,
        verts: *const f32,
        tcoords: *const f32,
        colors: *const std::os::raw::c_uint,
        nverts: std::os::raw::c_int,
    ) {
        log::info!("fontstash: draw");
        let mut me = &mut *(uptr as *const _ as *mut Self);
    }

    /// Free user texture data here
    unsafe extern "C" fn delete(uptr: *mut std::os::raw::c_void) {
        log::info!("fontstash: delete");
        let mut me = &mut *(uptr as *const _ as *mut Self);
        if !me.texture.is_null() {
            me.device.add_dispose_texture(me.texture);
        }
    }
}
