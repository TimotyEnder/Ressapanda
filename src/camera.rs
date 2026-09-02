use std::sync::Arc;

use cgmath::{Point3, Vector2, Vector3, prelude::*};
use winit::window::Window;

use crate::state::State;
pub struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}
impl Camera {
    pub fn new(
        eye: cgmath::Point3<f32>,
        target: cgmath::Point3<f32>,
        up: cgmath::Vector3<f32>,
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            eye,
            target,
            up,
            aspect,
            fovy,
            znear,
            zfar,
        }
    }
    pub fn get_eye_position(&self) -> cgmath::Point3<f32> {
        self.eye
    }
    pub fn set_position(&mut self, target: Point3<f32>, eye: Point3<f32>) {
        self.target = target;
        self.eye = eye;
    }
    pub fn right(&self) -> Vector3<f32> {
        (self.target - self.eye)
            .normalize()
            .cross(self.up)
            .normalize()
    }
    pub fn up_local(&self) -> Vector3<f32> {
        self.right().cross((self.target - self.eye).normalize())
    }
    pub fn get_fovy(&self) -> f32 {
        self.fovy
    }
    pub fn get_aspect(&self) -> f32 {
        self.aspect
    }
    pub fn get_z_near(&self) -> f32 {
        self.znear
    }
    pub fn get_z_far(&self) -> f32 {
        self.zfar
    }
    pub fn update_aspect(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    eye: [f32; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
            eye: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        let eye_pos = camera.get_eye_position();
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).into();
        self.eye = [eye_pos.x, eye_pos.y, eye_pos.z, 0.0];
    }
}
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);
