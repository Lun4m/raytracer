use std::{f32::consts::FRAC_PI_2, f32::consts::PI};

use bytemuck::{Pod, Zeroable};

use crate::algebra::Vec3;

// State shared with GPU
#[derive(Debug, Default, Clone, Copy, Pod, Zeroable)]
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

#[derive(Debug, Default)]
pub struct Camera {
    uniforms: CameraUniforms,
    // Point of camera focus
    center: Vec3,
    // Up direction
    up: Vec3,
    // Spherical coords
    //
    // Rotation angle along y-axis, [0, 2pi)
    azimuth: f32,
    // rotation angle around u [-pi/2, pi/2)
    altitude: f32,
    // Distance between origin and center
    distance: f32,
}

impl Camera {
    pub fn with_spherical_coords(
        center: Vec3,
        up: Vec3,
        distance: f32,
        azimuth: f32,
        altitude: f32,
    ) -> Camera {
        let mut camera = Camera {
            uniforms: CameraUniforms::zeroed(),
            up,
            center,
            azimuth,
            altitude,
            distance,
        };
        camera.calculate_uniforms();
        camera
    }

    pub fn uniforms(&self) -> &CameraUniforms {
        &self.uniforms
    }

    pub fn zoom(&mut self, displacement: f32) {
        self.uniforms.origin += displacement * self.uniforms.w;
        self.distance = (self.distance - displacement).abs();
    }

    pub fn pan<T: Into<f32>>(&mut self, du: T, dv: T) {
        let displacement = du.into() * self.uniforms.u + dv.into() * self.uniforms.v;
        self.uniforms.origin += displacement;
        self.center += displacement;
    }

    const EPS: f32 = 1e-6;
    // Only works if up is (0,+-1,0)
    const MAX_ALT: f32 = FRAC_PI_2 - Camera::EPS;
    const MAX_AZI: f32 = 2.0 * PI - Camera::EPS;

    pub fn orbit<T: Into<f32>>(&mut self, du: T, dv: T) {
        self.altitude = (self.altitude + dv.into()).clamp(-Camera::MAX_ALT, Camera::MAX_ALT);

        self.azimuth = {
            let mut new = self.azimuth + du.into();
            if new > Camera::MAX_AZI {
                new -= Camera::MAX_AZI;
            }
            new
        };

        println!("alt={} azi={}", self.altitude, self.azimuth);
        self.calculate_uniforms();
    }

    pub fn calculate_uniforms(&mut self) {
        let w = {
            let (sin_alt, cos_alt) = self.altitude.sin_cos();
            let (sin_azi, cos_azi) = self.azimuth.sin_cos();

            -Vec3::new(cos_alt * cos_azi, sin_alt, cos_alt * sin_azi)
        };
        let u = w.cross(&self.up).normalized();
        let v = u.cross(&w);

        let origin = self.center - self.distance * w;

        self.uniforms.origin = origin;
        self.uniforms.u = u;
        self.uniforms.v = v;
        self.uniforms.w = w;
    }
}
