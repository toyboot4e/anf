/*! Object-oriented draw APIs

[`DrawContext`] is the primary interface. It's recommended to do `use anf::engine::draw::*`.

* TODO: better flushing
* TODO: offscreen rendering
!*/

// re-export draw traits
pub use anf_gfx::cmd::traits::*;

use {
    anf_gfx::{
        batcher::{
            bufspecs::{ColoredVertexData, QuadData},
            Batcher,
        },
        cmd::{QuadParams, QuadPush, SpritePush},
        geom2d::*,
    },
    fna3d::{self, Color, Device},
    fna3d_hie::Pipeline,
    std::time::Duration,
};

use crate::gfx::TextureData2d;

static mut WHITE_DOT: Option<TextureData2d> = None;

/// The imperative draw API
///
/// Owns FNA3D device. Batches draw calls are automatically.
///
/// This type should be loved by users. If not.. please let me know!
///
/// # Example
///
/// ```no_run
/// use anf::engine::draw::*;
/// use anf::gfx::TextureData2d;
///
/// fn render(dcx: &mut DrawContext, tex: &TextureData2d) {
///     let mut pass = dcx.pass(); // batch pass
///     pass.texture(tex).dest_pos_px([100.0, 100.0]); // push texture
///     pass.texture(tex).dest_pos_px([100.0, 400.0]);
///     // drop(pass);
/// }
/// ```
#[derive(Debug)]
pub struct DrawContext {
    // states
    batcher: Batcher,
    pipe: Pipeline,
    push: QuadParams,
    /// Dependency
    device: Device,
    /// Dependency
    params: fna3d::PresentationParameters,
    /// Interface
    dt: Duration,
}

impl DrawContext {
    pub fn new(
        mut device: Device,
        default_shader_bytes: &[u8],
        params: fna3d::PresentationParameters,
    ) -> Self {
        let pipe = Pipeline::new(&mut device, ColoredVertexData::decl(), default_shader_bytes);
        let batcher = Batcher::from_device(&mut device);

        unsafe {
            let white_dot =
                TextureData2d::from_encoded_bytes(&device, crate::engine::embedded::WHITE_DOT)
                    .unwrap();
            WHITE_DOT = Some(white_dot);
        }

        Self {
            device,
            batcher,
            pipe,
            push: QuadParams::default(),
            params,
            dt: Duration::default(),
        }
    }

    pub fn raw_window(&self) -> *mut sdl2::sys::SDL_Window {
        self.params.deviceWindowHandle as *mut _
    }
}

/// Context
impl DrawContext {
    pub fn device(&mut self) -> &Device {
        &self.device
    }

    /// TODO: remove this
    pub fn set_dt(&mut self, ts: Duration) {
        self.dt = ts;
    }
}

/// Batcher
impl DrawContext {
    pub fn next_quad_mut_safe(&mut self, t: *mut fna3d::Texture) -> &mut QuadData {
        self.batcher.next_quad_mut(t, &self.device, &mut self.pipe)
    }

    pub fn flush(&mut self) {
        self.batcher.flush(&mut self.device, &mut self.pipe);
    }
}

/// Draw interface
impl DrawContext {
    /// Begins a batch pass, rendering with particular set of state
    pub fn pass(&mut self) -> BatchPass<'_> {
        BatchPass::new(self)
    }

    pub fn screen(&self) -> Rect2f {
        [
            0.0,
            0.0,
            self.params.backBufferWidth as f32,
            self.params.backBufferHeight as f32,
        ]
        .into()
    }

    pub fn dt(&self) -> Duration {
        self.dt
    }
}

/// Handle to push sprites
///
/// Binds a set of state for rendering and flushes the [`SpriteBatch`] when it goes out of scope.
///
/// "Batch pass" is not a common word but I think it makes sence.
///
/// Currently it doesn't handle those state such as render taret.
///
/// [`SpriteBatch`]: anf_gfx::batcher::batch::SpriteBatch
pub struct BatchPass<'a> {
    dcx: &'a mut DrawContext,
}

/// Flush batch when it goes out of scope
impl<'a> Drop for BatchPass<'a> {
    fn drop(&mut self) {
        self.dcx
            .batcher
            .flush(&mut self.dcx.device, &mut self.dcx.pipe);
    }
}

impl<'a> BatchPass<'a> {
    pub fn new(dcx: &'a mut DrawContext) -> Self {
        Self { dcx }
    }
}

/// Draw commands
///
/// TODO: consider using `Render` trait
impl<'a> BatchPass<'a> {
    fn next_push_mut(&mut self, tex: &impl Texture2d) -> QuadPush<'_> {
        let target =
            self.dcx
                .batcher
                .next_quad_mut(tex.raw_texture(), &self.dcx.device, &mut self.dcx.pipe);

        QuadPush {
            params: &mut self.dcx.push,
            target,
        }
    }

    /// Push texture or sprite
    pub fn push<S>(&mut self, sprite: &S) -> SpritePush
    where
        S: OnSpritePush + Texture2d,
    {
        SpritePush::new(self.next_push_mut(sprite), sprite)
    }
}

/// Outline drawing
impl<'a> BatchPass<'a> {
    pub fn white_dot(&mut self) -> SpritePush {
        unsafe { self.push(WHITE_DOT.as_ref().unwrap()) }
    }

    pub fn line(&mut self, p1: impl Into<Vec2f>, p2: impl Into<Vec2f>, color: Color) {
        let p1 = p1.into();
        let p2 = p2.into();

        let delta = p2 - p1;
        let rad = delta.rad();
        let len = delta.len();

        self.white_dot()
            .color(color)
            .dest_rect_px([p1, (len, 1.0).into()])
            .rot(rad);
    }

    pub fn rect(&mut self, rect: impl Into<Rect2f>, color: Color) {
        let rect = rect.into();
        let (p1, p2, p3, p4) = (
            rect.left_up(),
            rect.right_up(),
            rect.right_down(),
            rect.left_down(),
        );

        self.line(p1, p2, color);
        self.line(p2, p3, color);
        self.line(p3, p4, color);
        // FIXME: allow p4 -> p1
        self.line(p1, p4, color);
    }
}
