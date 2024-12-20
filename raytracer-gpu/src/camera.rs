use bytemuck::{Pod, Zeroable};

use crate::algebra::Vec3;

// State shared with GPU
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct CameraUniforms {
    origin: Vec3,
    _pad0: u32,
    u: Vec3,
    _pad1: u32,
    v: Vec3,
    _pad2: u32,
    w: Vec3,
    _pad3: u32,
}

#[derive(Debug)]
pub struct Camera {
    uniforms: CameraUniforms,
}

impl Camera {
    pub fn look_at(origin: Vec3, center: Vec3, up: Vec3) -> Camera {
        let w = (center - origin).normalized();
        let u = w.cross(&up).normalized();
        let v = u.cross(&w);

        Camera {
            uniforms: CameraUniforms {
                origin,
                _pad0: 0,
                u,
                _pad1: 0,
                v,
                _pad2: 0,
                w,
                _pad3: 0,
            },
        }
    }

    pub fn uniforms(&self) -> &CameraUniforms {
        &self.uniforms
    }
}
