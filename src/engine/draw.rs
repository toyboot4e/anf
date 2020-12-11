/*!

Object-oriented draw APIs

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
    fna3d_hie::Pipeline,
    fna3h::{self, tex::Texture, win::PresentationParameters, Color, Device},
    std::time::Duration,
};

use crate::gfx::TextureData2d;

static mut WHITE_DOT: Option<TextureData2d> = None;

/// The imperative draw API
///
/// Owns FNA3D device. Batches draw calls are automatically.
#[derive(Debug)]
pub struct DrawContext {
    // states
    batcher: Batcher,
    pipe: Pipeline,
    push: QuadParams,
    /// Dependency
    device: Device,
    /// Dependency
    params: PresentationParameters,
    /// Interface
    dt: Duration,
}

impl DrawContext {
    pub fn new(
        mut device: Device,
        default_shader_bytes: &[u8],
        params: PresentationParameters,
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
    pub fn flush(&mut self) {
        self.batcher.flush(&mut self.device, &mut self.pipe);
    }

    pub fn next_quad_mut(&mut self, t: *mut Texture) -> &mut QuadData {
        self.batcher.next_quad_mut(t, &self.device, &mut self.pipe)
    }
}

/// Draw interface
impl DrawContext {
    /// Begins a batch pass, rendering with particular set of state
    pub fn batch(&mut self) -> BatchPass<'_> {
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

pub trait DrawApi {
    /// Modify the quad manually!
    fn next_quad_mut(&mut self, t: *mut Texture) -> &mut QuadData;

    /// Modify the quad manually!
    fn next_push_mut(&mut self, tex: &impl Texture2d) -> QuadPush<'_>;

    /// Push texture or sprite, modifying the quad with builder
    fn push<S: OnSpritePush + Texture2d>(&mut self, sprite: &S) -> SpritePush {
        SpritePush::new(self.next_push_mut(sprite), sprite)
    }

    /// (Mainly) internal utilitiy to implement `linep and `rect`
    fn white_dot(&mut self) -> SpritePush {
        unsafe { self.push(WHITE_DOT.as_ref().unwrap()) }
    }

    fn line(&mut self, p1: impl Into<Vec2f>, p2: impl Into<Vec2f>, color: Color) {
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

    fn rect(&mut self, rect: impl Into<Rect2f>, color: Color) {
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

/// Handle to push sprites
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

impl<'a> DrawApi for BatchPass<'a> {
    fn next_quad_mut(&mut self, t: *mut Texture) -> &mut QuadData {
        self.dcx.next_quad_mut(t)
    }

    fn next_push_mut(&mut self, tex: &impl Texture2d) -> QuadPush<'_> {
        // we have to take care into ownership, unforunatelly
        let target_quad =
            self.dcx
                .batcher
                .next_quad_mut(tex.raw_texture(), &self.dcx.device, &mut self.dcx.pipe);

        QuadPush {
            params: &mut self.dcx.push,
            target: target_quad,
        }
    }
}

pub struct OffscreenPass<'a, 'b, S: OnSpritePush + Texture2d> {
    dcx: &'a mut DrawContext,
    target: fna3h::draw::pass::RenderTargetBinding,
    sprite: &'b S,
}

// pub enum RenderPass<'a> {
//     BackBuffer {
//         dcx: &'a mut DrawContext,
//         // scrissor, viewport
//     },
//     Offscreen{
//         dcx: &'a mut DrawContext,
//         target: fna3h::draw::pass::RenderTargetBinding,
//     },
// }
