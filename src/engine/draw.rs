//! Object-oriented draw APIs
//!
//! [`DrawContext`] is the primary interface. `use anf::engine::draw::*` is the recommended.

const WHITE_DOT: &[u8] = include_bytes!("white_dot.png");

pub use anf_gfx::cmd::prelude::*;
use fna3d::Color;

use std::path::Path;

use anf_gfx::{
    batcher::{bufspecs::ColoredVertexData, Batcher},
    cmd::{QuadPush, QuadPushBinding, SpritePushCommand},
    geom2d::*,
};

use fna3d::{self, Device};
use fna3d_hie::Pipeline;

use crate::{engine::time::TimeStep, gfx::TextureData2d};

/// The imperative draw API
///
/// Batches draw calls automatically. Owns FNA3D device.
///
/// This type should be loved by users. If you don't.. please let me know!
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
/// }
/// ```
#[derive(Debug)]
pub struct DrawContext {
    // states
    batcher: Batcher,
    pipe: Pipeline,
    push: QuadPush,
    // builtin
    white_dot: TextureData2d,
    /// dependency
    device: Device,
    /// dependency
    params: fna3d::PresentationParameters,
    /// interface
    time_step: TimeStep,
}

impl DrawContext {
    pub fn new(
        mut device: Device,
        default_shader: impl AsRef<Path>,
        params: fna3d::PresentationParameters,
    ) -> Self {
        let pipe = Pipeline::new(&mut device, ColoredVertexData::decl(), default_shader);
        let batcher = Batcher::from_device(&mut device);
        let white_dot = TextureData2d::from_undecoded_bytes(&mut device, WHITE_DOT).unwrap();
        Self {
            device,
            batcher,
            pipe,
            white_dot,
            push: QuadPush::default(),
            params,
            time_step: TimeStep::default(),
        }
    }

    pub fn raw_window(&self) -> *mut sdl2::sys::SDL_Window {
        self.params.deviceWindowHandle as *mut _
    }

    pub fn device_mut(&mut self) -> &mut Device {
        &mut self.device
    }

    /// TODO: remove this
    pub fn set_time_step(&mut self, ts: TimeStep) {
        self.time_step = ts;
    }
}

impl AsRef<fna3d::Device> for DrawContext {
    fn as_ref(&self) -> &fna3d::Device {
        &self.device
    }
}

impl AsMut<fna3d::Device> for DrawContext {
    fn as_mut(&mut self) -> &mut fna3d::Device {
        &mut self.device
    }
}

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

    pub fn dt_secs_f32(&self) -> f32 {
        self.time_step.dt_secs_f32()
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
            .end(&mut self.dcx.device, &mut self.dcx.pipe);
    }
}

impl<'a> BatchPass<'a> {
    pub fn new(dcx: &'a mut DrawContext) -> Self {
        dcx.batcher.begin();
        Self { dcx }
    }

    /// Creates [`SpritePushCommand`] using [`SubTexture2d`] attributes
    pub fn texture<T: SubTexture2d>(&mut self, texture: T) -> SpritePushCommand<'_, T> {
        if self.dcx.batcher.is_satured() {
            self.dcx
                .batcher
                .flush(&mut self.dcx.device, &mut self.dcx.pipe);
            self.dcx.batcher.begin();
        }

        self.dcx.push.reset_to_defaults();
        let quad = QuadPushBinding {
            push: &mut self.dcx.push,
            batch: &mut self.dcx.batcher.batch,
        };
        SpritePushCommand::from_sub_texture(quad, texture)
    }

    /// Creates [`SpritePushCommand`] using [`Sprite`] attributes
    pub fn sprite<T: Sprite>(&mut self, sprite: T) -> SpritePushCommand<'_, T> {
        if self.dcx.batcher.is_satured() {
            self.dcx
                .batcher
                .flush(&mut self.dcx.device, &mut self.dcx.pipe);
            self.dcx.batcher.begin();
        }

        self.dcx.push.reset_to_defaults();
        let quad = QuadPushBinding {
            push: &mut self.dcx.push,
            batch: &mut self.dcx.batcher.batch,
        };
        SpritePushCommand::from_sprite(quad, sprite)
    }

    // TODO: add wrapper of primitive renderer
    pub fn white_dot(&mut self) -> SpritePushCommand<'_, TextureData2d> {
        self.texture(self.dcx.white_dot.clone())
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
