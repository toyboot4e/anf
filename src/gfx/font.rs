//! Font book on font stash

pub use ::fontstash::{self, FonsTextIter, FontStash};

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
            stash: FontStash::uninitialized(),
            device,
            texture: std::ptr::null_mut(),
            w,
            h,
            is_dirty: true,
        });

        let inner_ptr = inner.as_ref() as *const _ as *mut FontBookInternal;
        inner.stash.init_mut(w, h, inner_ptr);
        fontstash::set_error_callback(
            inner.stash().raw(),
            fons_error_callback,
            inner_ptr as *mut _,
        );

        return FontBook { inner };

        unsafe extern "C" fn fons_error_callback(
            _uptr: *mut ::std::os::raw::c_void,
            error_code: ::std::os::raw::c_int,
            _val: ::std::os::raw::c_int,
        ) {
            let error = match fontstash::ErrorCode::from_u32(error_code as u32) {
                Some(error) => error,
                None => {
                    log::warn!("fons error error: given broken erroor code");
                    return;
                }
            };

            // log::warn!("fons error: {:?}", error);
        }
    }
}

/// The internals of [`FontBook`]
///
/// The memory location should be fixed.
pub struct FontBookInternal {
    stash: fontstash::FontStash,
    device: fna3d::Device,
    /// The texture is always valid
    texture: *mut fna3d::Texture,
    /// The texture size is always synced with the fontstash size
    w: u32,
    /// The texture size is always synced with the fontstash size
    h: u32,
    /// Shall we update the texture data?
    is_dirty: bool,
}

impl Drop for FontBookInternal {
    fn drop(&mut self) {
        log::trace!("fontbook: drop");

        if !self.texture.is_null() {
            self.device.add_dispose_texture(self.texture);
        }
    }
}

impl FontBookInternal {
    pub fn texture(&self) -> *mut fna3d::Texture {
        self.texture
    }

    pub fn stash(&self) -> FontStash {
        self.stash.clone()
    }

    /// TTODO: resize when needed
    pub fn text_iter(&mut self, text: &str) -> fontstash::Result<FonsTextIter> {
        let iter = self.stash.text_iter(text)?;
        self.update_texture();
        Ok(iter)
    }

    /// Updates GPU texure. Call it whenever drawing text
    fn update_texture(&mut self) {
        // TODO: correct dirty test
        // let (is_dirty, _dirty_rect) = self.stash.dirty();
        if !self.is_dirty {
            return;
        }
        self.is_dirty = false;

        log::trace!("fontbook: update font texture");

        self.stash.with_pixels(|pixels, w, h| {
            // need this?

            let data = {
                log::trace!("=> size: [{}, {}]", w, h);
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

/// Renderer implementation
///
/// Return `1` to represent success.
unsafe impl fontstash::Renderer for FontBookInternal {
    /// Creates font texture
    unsafe extern "C" fn create(
        uptr: *mut std::os::raw::c_void,
        width: std::os::raw::c_int,
        height: std::os::raw::c_int,
    ) -> std::os::raw::c_int {
        log::trace!("fontbook: create ([{}, {}])", width, height);
        let me = &mut *(uptr as *const _ as *mut Self);

        if !me.texture.is_null() {
            me.device.add_dispose_texture(me.texture);
        }

        me.texture = me.device.create_texture_2d(
            fna3d::SurfaceFormat::Color,
            width as u32,
            height as u32,
            0,
            false,
        );
        me.w = width as u32;
        me.h = height as u32;

        me.is_dirty = true;

        return 1;
    }

    unsafe extern "C" fn resize(
        uptr: *mut std::os::raw::c_void,
        width: std::os::raw::c_int,
        height: std::os::raw::c_int,
    ) -> std::os::raw::c_int {
        log::trace!("fontbook: resize");
        Self::create(uptr, width, height);
        1 // success
    }

    /// Try to resize texture while the atlas is full
    unsafe extern "C" fn expand(uptr: *mut std::os::raw::c_void) -> std::os::raw::c_int {
        log::trace!("fontbook: expand");
        let me = &mut *(uptr as *const _ as *mut Self);
        if let Err(why) = me.stash.expand_atlas(me.w * 2, me.h * 2) {
            log::warn!("fontstash: error on resize: {:?}", why);
            0 // fail
        } else {
            1
        }
    }

    unsafe extern "C" fn update(
        uptr: *mut std::os::raw::c_void,
        rect: *mut std::os::raw::c_int,
        data: *const std::os::raw::c_uchar,
    ) -> std::os::raw::c_int {
        log::trace!("fontbook: update");
        // FIXME: is this called?
        // TODO: make use of rect and data
        let me = &mut *(uptr as *const _ as *mut Self);
        me.is_dirty = true;
        // me.update_texture();
        1
    }
}
