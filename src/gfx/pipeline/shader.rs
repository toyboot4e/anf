//! Also known as `Effect` in FNA

use std::{
    fs,
    io::{self, Read},
};

/// Part of the required rendering pipeline
///
/// * TODO: is this vertex shader or fragment shader?
#[derive(Debug)]
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

    /// A requierd rendering pipeline cycle
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

    /// A requierd rendering pipeline cycle
    ///
    /// Inlined `OrthographicOffCenter` matrix
    pub fn update(&mut self) {
        unsafe {
            for i in 0..(*self.mojo_effect).param_count as isize {
                let name = (*(*self.mojo_effect).params.offset(i)).value.name;
                // FIXME: do not allocate a new string..
                let compared = std::ffi::CString::new("MatrixTransform").unwrap();
                if std::ffi::CStr::from_ptr(name) == compared.as_c_str() {
                    // OrthographicOffCenter Matrix - value copied from XNA project
                    // todo: Do I need to worry about row-major/column-major?
                    let proj_mat: [f32; 16] = [
                        0.0015625 as f32,
                        0 as f32,
                        0 as f32,
                        -1 as f32,
                        0 as f32,
                        -0.00277777785 as f32,
                        0 as f32,
                        1 as f32,
                        0 as f32,
                        0 as f32,
                        1 as f32,
                        0 as f32,
                        0 as f32,
                        0 as f32,
                        0 as f32,
                        1 as f32,
                    ];

                    use std::io::Write;
                    let len = std::mem::size_of::<f32>() * 16;
                    let mut dest = std::slice::from_raw_parts_mut(
                        (*(*self.mojo_effect).params.offset(i))
                            .value
                            .__bindgen_anon_1
                            .values as *mut u8,
                        len,
                    );
                    let src = std::slice::from_raw_parts_mut(proj_mat.as_ptr() as *mut u8, len);
                    dest.write(src)
                        .expect("failed to write universal effect data");

                    break; // TODO: why break. look at FNA
                }
            }
        }
    }
}
