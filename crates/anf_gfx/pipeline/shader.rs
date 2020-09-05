//! Re-exported to super module

use anf_deps::fna3d;
use std::{
    fs,
    io::{self, Read},
    path::Path,
};

/// Shader data loaded on memory
///
/// In XNA, shaders are known as [`Effect`s.
///
/// * TODO: how to write a custom shader
/// * TODO: enable loading custom shaders, maybe using `mint`?
/// * TODO: is this vertex shader or fragment shader?
/// * TODO: drop
#[derive(Debug)]
pub struct Shader {
    effect: *mut fna3d::Effect,
    mojo_effect: *mut fna3d::mojo::Effect,
}

impl Shader {
    pub fn from_device(
        device: &mut fna3d::Device,
        shader_path: impl AsRef<Path>,
    ) -> io::Result<Self> {
        let mut f = fs::File::open(shader_path)?;
        let mut buf = Vec::new();
        let len = f.read_to_end(&mut buf)?; // TODO: use anyhow or like that

        let (effect, mojo_effect) = device.create_effect(buf.as_mut_ptr(), len as u32);

        unsafe {
            let mojo_effect: &mut fna3d::mojo::Effect = &mut *mojo_effect;

            if mojo_effect.error_count > 0 {
                let errs = std::slice::from_raw_parts(
                    mojo_effect.techniques,
                    mojo_effect.technique_count as usize,
                );
                eprintln!("{:?}", errs);
                // TODO: error?
            }
        }

        Ok(Self {
            effect,
            mojo_effect,
        })
    }
}

/// Rendering pipeline methods
/// ---
impl Shader {
    /// * `FNA3D_ApplyEffect`
    ///
    /// * TODO: what is `pass`? is it actually typed?
    pub fn apply_effect(&mut self, device: &mut fna3d::Device, pass: u32) {
        // no effect state change
        let state_changes = fna3d::mojo::EffectStateChanges {
            render_state_change_count: 0,
            render_state_changes: std::ptr::null(),
            sampler_state_change_count: 0,
            sampler_state_changes: std::ptr::null(),
            vertex_sampler_state_change_count: 0,
            vertex_sampler_state_changes: std::ptr::null(),
        };
        device.apply_effect(self.effect, pass, &state_changes);
    }

    /// * TODO: enable custom projection matrix
    pub fn apply_uniforms(&mut self) {
        fna3d::mojo::set_projection_uniform(self.mojo_effect, &fna3d::mojo::ORTHOGRAPIHCS_MATRIX);
    }
}
