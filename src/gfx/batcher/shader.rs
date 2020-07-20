//! Also known as `Effect` in FNA
//!
//! It's a part of the rendering pipeline and cannot be skipped.

use std::{
    env, fs,
    io::{self, Read},
    path::PathBuf,
};

pub struct Shader {
    effect: *mut fna3d::Effect,
    mojo_effect: *mut fna3d::mojo::Effect,
}

impl Shader {
    pub fn from_device(device: &mut fna3d::Device) -> io::Result<Self> {
        let path = crate::vfs::default_shader();
        log::trace!("default shader located at {}", path.display());

        let mut f = fs::File::open(&path)?;
        let mut buf = Vec::new();
        let len = f.read_to_end(&mut buf)?; // TODO: use anyhow or like that

        // TODO: can we forget about `Effect`?
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

    // TODO: what is `pass`? typed?
    pub fn apply_effect(&mut self, device: &mut fna3d::Device, pass: u32) {
        // no effect state change
        let state_changes = fna3d::sys::mojo::MOJOSHADER_effectStateChanges {
            render_state_change_count: 0,
            render_state_changes: std::ptr::null(),
            sampler_state_change_count: 0,
            sampler_state_changes: std::ptr::null(),
            vertex_sampler_state_change_count: 0,
            vertex_sampler_state_changes: std::ptr::null(),
        };
        device.apply_effect(self.effect, pass, &state_changes);
    }
}
