//! ImGUI renderer implementation in FNA3D based on [the example]
//!
//! [the xample]: https://github.com/Gekkio/imgui-rs/blob/master/imgui-gfx-renderer/src/lib.rs
//!
//! FIXME: is it bad practice to use `raw_device` field because it may drop earlier than Device

use std::{mem::size_of, rc::Rc};

use thiserror::Error;

use imgui::{
    im_str, internal::RawWrapper, BackendFlags, DrawCmd, DrawCmdParams, FontConfig, FontSource,
};

// TODO: extend and use this error
#[derive(Debug, Error)]
pub enum ImGuiRendererError {
    #[error("bad texture id")]
    BadTexture(imgui::TextureId),
}

pub type Result<T> = std::result::Result<T, ImGuiRendererError>;

pub struct Texture2D {
    pub raw: *mut fna3d::Texture,
    raw_device: *mut fna3d::sys::FNA3D_Device,
    pub w: u32,
    pub h: u32,
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        let device = unsafe { &mut *(self.raw_device as *mut fna3d::Device) };
        device.add_dispose_texture(self.raw);
    }
}

pub struct RcTexture {
    pub texture: Rc<Texture2D>,
}

/// FNA3D ImGUI renderer
pub struct ImGuiRenderer {
    textures: imgui::Textures<RcTexture>,
    font_texture: RcTexture,
    batch: Batch,
}

impl ImGuiRenderer {
    /// Initializes the renderer with default configuration
    ///
    /// Based on: https://github.com/Gekkio/imgui-rs/blob/master/imgui-examples/examples/support/mod.rs
    pub fn quick_start(
        device: &mut fna3d::Device,
        display_size: [f32; 2],
        font_size: f32,
        hidpi_factor: f32,
    ) -> Result<(imgui::Context, ImGuiRenderer)> {
        let mut icx = imgui::Context::create();

        // initial window setting
        icx.io_mut().display_size = display_size;

        // setting up fonts
        {
            let font_size = (font_size * hidpi_factor) as f32;
            icx.fonts().add_font(&[
                FontSource::DefaultFontData {
                    config: Some(FontConfig {
                        size_pixels: font_size,
                        ..FontConfig::default()
                    }),
                },
                FontSource::TtfData {
                    data: crate::JP_FONT,
                    size_pixels: font_size,
                    config: Some(FontConfig {
                        rasterizer_multiply: 1.75,
                        glyph_ranges: imgui::FontGlyphRanges::japanese(),
                        ..FontConfig::default()
                    }),
                },
            ]);
            icx.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        }

        let renderer = ImGuiRenderer::init(&mut icx, device)?;

        Ok((icx, renderer))
    }

    /// Add font before loading
    pub fn init(icx: &mut imgui::Context, device: &mut fna3d::Device) -> Result<Self> {
        icx.set_renderer_name(Some(im_str!(
            "imgui-fna3d-renderer {}",
            env!("CARGO_PKG_VERSION")
        )));

        icx.io_mut()
            .backend_flags
            .insert(BackendFlags::RENDERER_HAS_VTX_OFFSET);

        let font_texture = Self::load_font_texture(device, icx.fonts())?;

        Ok(ImGuiRenderer {
            textures: imgui::Textures::new(),
            font_texture,
            batch: Batch::new(device),
        })
    }

    /// Be warned that the font texture is  non-premultiplied alpha
    fn load_font_texture(
        device: &mut fna3d::Device,
        mut fonts: imgui::FontAtlasRefMut,
    ) -> Result<RcTexture> {
        let atlas_texture = fonts.build_rgba32_texture();
        let (pixels, w, h) = (
            atlas_texture.data,
            atlas_texture.width,
            atlas_texture.height,
        );

        log::info!("fna3d-imgui-rs font texture size: ({}, {})", w, h);

        // create GPU texture
        let raw = {
            let fmt = fna3d::SurfaceFormat::Color;
            let level = 0; // no mipmap
            let gpu_texture = device.create_texture_2d(fmt, w, h, level, false);
            device.set_texture_data_2d(gpu_texture, 0, 0, w, h, level, pixels);

            gpu_texture
        };

        let font_texture = Texture2D {
            raw,
            raw_device: device.raw(),
            w,
            h,
        };

        // Note that we have to set the ID *AFTER* creating the font atlas texture
        fonts.tex_id = imgui::TextureId::from(usize::MAX);

        Ok(RcTexture {
            texture: Rc::new(font_texture),
        })
    }

    pub fn textures_mut(&mut self) -> &mut imgui::Textures<RcTexture> {
        &mut self.textures
    }

    /// Be warned that the font texture is  non-premultiplied alpha
    pub fn font_texture(&self) -> &Texture2D {
        &self.font_texture.texture
    }

    /// https://en.wikipedia.org/wiki/Orthographic_projection
    fn ortho_matrix(draw_data: &imgui::DrawData) -> [f32; 16] {
        let left = draw_data.display_pos[0];
        let right = draw_data.display_pos[0] + draw_data.display_size[0];
        let top = draw_data.display_pos[1];
        let bottom = draw_data.display_pos[1] + draw_data.display_size[1];

        // TODO: maybe use [[f32; 4]; 4]]
        [
            (2.0 / (right - left)),
            0.0,
            0.0,
            -(right + left) / (right - left),
            //
            0.0,
            2.0 / (top - bottom),
            0.0,
            -(top + bottom) / (top - bottom),
            //
            0.0,
            0.0,
            -1.0, // FIXME: is this right? (far/near plane)
            0.0,
            //
            0.0,
            0.0,
            0.0,
            1.0,
        ]
    }

    /// Set render target to FNA3D device before/after calling this method
    pub fn render(
        &mut self,
        draw_data: &imgui::DrawData,
        device: &mut fna3d::Device,
    ) -> Result<()> {
        let fb_width = draw_data.display_size[0] * draw_data.framebuffer_scale[0];
        let fb_height = draw_data.display_size[1] * draw_data.framebuffer_scale[1];

        if fb_width <= 0.0 || fb_height <= 0.0 {
            return Ok(());
        }

        // TODO: restore/restore previous state
        device.set_blend_state(&fna3d::BlendState::non_premultiplied());
        // SamplerState.LinearWrap;
        // DepthStencilState.None;
        // RasterizerState = RasterizerState.CullNone;

        log::trace!("fna3d-imgui-rs: start rendering");

        let matrix = Self::ortho_matrix(draw_data);
        fna3d::mojo::set_projection_matrix(self.batch.effect_data, &matrix);

        let clip_off = draw_data.display_pos;
        let clip_scale = draw_data.framebuffer_scale;

        for draw_list in draw_data.draw_lists() {
            self.batch.set_draw_list(draw_list, device);

            for cmd in draw_list.commands() {
                match cmd {
                    DrawCmd::Elements {
                        count, // this is actually `n_indices`
                        cmd_params:
                            DrawCmdParams {
                                clip_rect,
                                texture_id,
                                vtx_offset,
                                idx_offset,
                            },
                    } => {
                        let clip_rect = [
                            (clip_rect[0] - clip_off[0]) * clip_scale[0],
                            (clip_rect[1] - clip_off[1]) * clip_scale[1],
                            (clip_rect[2] - clip_off[0]) * clip_scale[0],
                            (clip_rect[3] - clip_off[1]) * clip_scale[1],
                        ];

                        // FIXME:
                        if clip_rect[0] >= fb_width
                            || clip_rect[1] >= fb_height
                            || clip_rect[2] < 0.0
                            || clip_rect[3] < 0.0
                        {
                            // skip
                        } else {
                            // draw

                            let texture = if texture_id.id() == usize::MAX {
                                &self.font_texture
                            } else {
                                log::trace!("texture id {:?}", texture_id);
                                self.textures
                                    .get(texture_id)
                                    .ok_or_else(|| ImGuiRendererError::BadTexture(texture_id))?
                            };

                            let scissors_rect = fna3d::Rect {
                                x: f32::max(0.0, clip_rect[0]).floor() as i32,
                                y: f32::max(0.0, clip_rect[1]).floor() as i32,
                                w: (clip_rect[2] - clip_rect[0]).abs().ceil() as i32,
                                h: (clip_rect[3] - clip_rect[1]).abs().ceil() as i32,
                            };

                            self.batch.prepare_draw(
                                device,
                                &scissors_rect,
                                texture.texture.raw,
                                vtx_offset as u32,
                            );

                            // TODO: what is that count. indices?
                            log::trace!(
                                "draw (count: {}, vtx_offset: {}, idx_offset: {})",
                                count,
                                vtx_offset,
                                idx_offset
                            );

                            // `count` is actually `n_indices`
                            let n_primitives = count / 3;
                            device.draw_indexed_primitives(
                                fna3d::PrimitiveType::TriangleList,
                                vtx_offset as u32,
                                idx_offset as u32,
                                n_primitives as u32,
                                self.batch.ibuf.buf,
                                fna3d::IndexElementSize::Bits16,
                            );
                        }
                    }
                    DrawCmd::ResetRenderState => {
                        log::info!("fna3d-imgui-rs: ResetRenderState not implemented");
                        // TODO: what?
                    }
                    DrawCmd::RawCallback { callback, raw_cmd } => unsafe {
                        callback(draw_list.raw(), raw_cmd)
                    },
                }
            }
        }

        log::trace!("fna3d-imgui-rs: finish rendering");

        Ok(())
    }
}

// --------------------------------------------------------------------------------
// Batch

/// Buffer of GPU buffers
///
/// Drops internal buffers automatically.
struct Batch {
    raw_device: *mut fna3d::sys::FNA3D_Device,
    ibuf: GpuIndexBuffer,
    vbuf: GpuVertexBuffer,
    effect: *mut fna3d::Effect,
    effect_data: *mut fna3d::mojo::Effect,
}

impl Drop for Batch {
    fn drop(&mut self) {
        let device = unsafe { &mut *(self.raw_device as *mut fna3d::Device) };
        device.add_dispose_index_buffer(self.ibuf.buf);
        device.add_dispose_vertex_buffer(self.vbuf.buf);
        device.add_dispose_effect(self.effect);
    }
}

impl Batch {
    fn new(device: &mut fna3d::Device) -> Self {
        const N_QUADS: usize = 2048; // buffers are pre-allocated for this number
        let vbuf = GpuVertexBuffer::new(device, 4 * N_QUADS); // four vertices per quad
        let ibuf = GpuIndexBuffer::new(device, 6 * N_QUADS); // six indices per quad

        let (effect, effect_data) =
            fna3d::mojo::load_shader_from_bytes(device, crate::SHARDER).unwrap();
        fna3d::mojo::set_projection_matrix(effect_data, &fna3d::mojo::ORTHOGRAPHICAL_MATRIX);

        Self {
            raw_device: device.raw(),
            vbuf,
            ibuf,
            effect,
            effect_data,
        }
    }

    fn set_draw_list(&mut self, draw_list: &imgui::DrawList, device: &mut fna3d::Device) {
        self.vbuf.upload_vertices(&draw_list.vtx_buffer(), device);
        self.ibuf.upload_indices(&draw_list.idx_buffer(), device);
    }

    /// Sets up rendering pipeline before making a draw call
    fn prepare_draw(
        &mut self,
        device: &mut fna3d::Device,
        scissors_rect: &fna3d::Rect,
        texture: *mut fna3d::Texture,
        vtx_offset: u32,
    ) {
        device.set_scissor_rect(&scissors_rect);

        // apply effect
        let state_changes = fna3d::mojo::EffectStateChanges {
            render_state_change_count: 0,
            render_state_changes: std::ptr::null(),
            sampler_state_change_count: 0,
            sampler_state_changes: std::ptr::null(),
            vertex_sampler_state_change_count: 0,
            vertex_sampler_state_changes: std::ptr::null(),
        };
        let pass = 0;
        device.apply_effect(self.effect, pass, &state_changes);

        // set texture
        let sampler = fna3d::SamplerState::linear_wrap();
        let slot = 0;
        device.verify_sampler(slot, texture, &sampler);

        // apply vertex buffer binding
        let bind = fna3d::VertexBufferBinding {
            vertexBuffer: self.vbuf.buf,
            vertexDeclaration: VERT_DECL,
            vertexOffset: 0, // FIXME:
            instanceFrequency: 0,
        };
        device.apply_vertex_buffer_bindings(&[bind], true, vtx_offset);
    }
}

struct GpuVertexBuffer {
    buf: *mut fna3d::Buffer,
    capacity_in_bytes: usize,
}

impl GpuVertexBuffer {
    fn new(device: &mut fna3d::Device, n_vertices: usize) -> Self {
        let len = VERT_SIZE * n_vertices;
        let buf = device.gen_vertex_buffer(true, fna3d::BufferUsage::None, len as u32);

        Self {
            buf,
            capacity_in_bytes: len,
        }
    }

    fn upload_vertices<T>(&mut self, data: &[T], device: &mut fna3d::Device) {
        // re-allocate if necessary
        // each index takes 20 bytes
        let len = VERT_SIZE * (data.len() + size_of::<T>()); // byte length
        if len > self.capacity_in_bytes {
            log::info!(
                "fna3d-imgui-rs: reallocate vertex buffer with byte length {}",
                len
            );
            device.add_dispose_vertex_buffer(self.buf);
            self.buf = device.gen_vertex_buffer(true, fna3d::BufferUsage::None, len as u32);
            self.capacity_in_bytes = len;
        }

        device.set_vertex_buffer_data(self.buf, 0, data, fna3d::SetDataOptions::None);
    }
}

struct GpuIndexBuffer {
    buf: *mut fna3d::Buffer,
    capacity_in_bytes: usize,
}

impl GpuIndexBuffer {
    fn new(device: &mut fna3d::Device, n_indices: usize) -> Self {
        let len = INDEX_SIZE * n_indices;
        let buf = device.gen_index_buffer(true, fna3d::BufferUsage::None, len as u32);

        Self {
            buf,
            capacity_in_bytes: len,
        }
    }

    fn upload_indices<T>(&mut self, data: &[T], device: &mut fna3d::Device) {
        // reallocate if necessary
        // each index takes 2 bytes (16 bits)
        let len = INDEX_SIZE * (data.len() + size_of::<T>()); // byte length
        if len > self.capacity_in_bytes {
            log::info!(
                "fna3d-imgui-rs: re-allocating index buffer with byte length {}",
                len
            );
            device.add_dispose_index_buffer(self.buf);
            self.buf = device.gen_index_buffer(true, fna3d::BufferUsage::None, len as u32);
            self.capacity_in_bytes = len;
        }

        device.set_index_buffer_data(self.buf, 0, data, fna3d::SetDataOptions::None);
    }
}

/// Size of a vertex in byte
const VERT_SIZE: usize = 20;

/// Size of an index in byte
const INDEX_SIZE: usize = 2;

/// Attributes of [`imgui::DrawVert`]
///
/// * pos: [f32; 2]
/// * uv: [f32; 2]
/// * col: [u8; 4]
const VERT_ELEMS: [fna3d::VertexElement; 3] = [
    fna3d::VertexElement {
        offset: 0,
        vertexElementFormat: fna3d::VertexElementFormat::Vector2 as u32,
        vertexElementUsage: fna3d::VertexElementUsage::Position as u32,
        usageIndex: 0,
    },
    fna3d::VertexElement {
        offset: 8,
        vertexElementFormat: fna3d::VertexElementFormat::Vector2 as u32,
        vertexElementUsage: fna3d::VertexElementUsage::TextureCoordinate as u32,
        usageIndex: 0,
    },
    fna3d::VertexElement {
        offset: 16,
        vertexElementFormat: fna3d::VertexElementFormat::Color as u32,
        vertexElementUsage: fna3d::VertexElementUsage::Color as u32,
        usageIndex: 0,
    },
];

const VERT_DECL: fna3d::VertexDeclaration = fna3d::VertexDeclaration {
    vertexStride: 20,
    elementCount: 3,
    elements: VERT_ELEMS.as_ptr() as *mut _,
};

#[cfg(test)]
mod test {
    use std::mem::size_of;

    #[test]
    fn test_size() {
        assert_eq!(size_of::<imgui::DrawVert>(), 20);
    }
}
