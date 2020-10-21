//! Re-exported to super module

use ::std::{
    ffi::{c_void, CStr},
    path::Path,
};

/// Shader data loaded on memory
///
/// In XNA, shaders are known as `Effect`s.
///
/// * TODO: how to write a custom shader
/// * TODO: enable loading custom shaders, maybe using `mint`?
/// * TODO: is this vertex shader or fragment shader?
/// * TODO: drop
#[derive(Debug)]
pub struct Shader {
    effect: *mut fna3d::Effect,
    data: *mut fna3d::mojo::Effect,
}

impl Shader {
    pub fn from_file(
        device: &fna3d::Device,
        shader_path: impl AsRef<Path>,
    ) -> fna3d::mojo::Result<Self> {
        let (effect, data) = fna3d::mojo::from_file(device, shader_path)?;
        Ok(Self { effect, data })
    }

    pub fn from_bytes(device: &fna3d::Device, shader_bytes: &[u8]) -> fna3d::mojo::Result<Self> {
        let (effect, data) = fna3d::mojo::from_bytes(device, shader_bytes)?;
        Ok(Self { effect, data })
    }

    pub fn destroy(self, device: &fna3d::Device) {
        // both (effect, data) are destroied:
        device.add_dispose_effect(self.effect);
    }
}

/// Rendering pipeline methods
/// ---
impl Shader {
    /// * `FNA3D_ApplyEffect`
    ///
    /// * TODO: what is `pass`? is it actually typed?
    pub fn apply_effect(&mut self, device: &fna3d::Device, pass: u32) {
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

    pub fn param(&self, name: &CStr) -> Option<*mut c_void> {
        fna3d::mojo::find_param(self.data, name)
    }

    pub unsafe fn set_param<T>(&self, name: &str, value: &T) {
        let name = std::ffi::CString::new(name).unwrap();
        fna3d::mojo::set_param(self.data, &name, value);
    }
}
