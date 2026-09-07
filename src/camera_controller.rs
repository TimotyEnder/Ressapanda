use std::f64;

use cgmath::{EuclideanSpace, Point3, Vector2, Vector3};
use winit::event::{MouseButton, MouseScrollDelta, TouchPhase};

use crate::{camera::Camera, conversion_utils::spherical_to_cartesian};

pub struct CameraController {
    sensitivity_rad_per_pixel: f64,
    rotate_started: bool,
    pan_started: bool,
    rotate_last_position: Option<Vector2<f64>>,
    pan_last_position: Option<Vector2<f64>>,
    yaw: f64,
    pitch: f64,
    radius: f64,
    pan_offset: Vector3<f32>,
}
impl CameraController {
    pub fn new(rotation_sensitivity: f32, camera: &Camera) -> Self {
        let radius = (camera.get_eye_position().x.powf(2.0)
            + camera.get_eye_position().y.powf(2.0)
            + camera.get_eye_position().z.powf(2.0))
        .sqrt();
        Self {
            sensitivity_rad_per_pixel: rotation_sensitivity as f64,
            rotate_started: false,
            pan_started: false,
            rotate_last_position: None,
            pan_last_position: None,
            yaw: 0.0,
            pitch: (camera.get_eye_position().y as f64 / radius as f64).atan(),
            radius: radius as f64,
            pan_offset: Vector3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn handle_mouse_button(&mut self, button: MouseButton, is_pressed: bool) {
        match button {
            MouseButton::Middle => {
                self.rotate_started = is_pressed && !self.pan_started;
                if !is_pressed {
                    self.rotate_last_position = None;
                }
            }
            MouseButton::Right => {
                self.pan_started = is_pressed && !self.rotate_started;
                if !is_pressed {
                    self.pan_last_position = None;
                }
            }
            _ => {}
        };
    }
    pub fn handle_mouse_position(&mut self, x: f64, y: f64, viewport_height: f32, camera: &Camera) {
        self.handle_rotation(x, y);
        self.handle_pan(x, y, camera, viewport_height);
    }
    fn handle_rotation(&mut self, x: f64, y: f64) {
        if !self.rotate_started {
            return;
        }
        let current = Vector2::new(x, y);
        if let Some(last_pos) = self.rotate_last_position {
            let delta = current - last_pos;
            self.yaw -= delta.x * self.sensitivity_rad_per_pixel;
            self.pitch += delta.y * self.sensitivity_rad_per_pixel;
            self.pitch = self
                .pitch
                .clamp(-f64::consts::PI / 2.0 + 0.01, f64::consts::PI / 2.0 - 0.01);
            self.rotate_last_position = Some(current);
        } else {
            self.rotate_last_position = Some(current);
        }
    }
    fn handle_pan(&mut self, x: f64, y: f64, camera: &Camera, viewport_height: f32) {
        if !self.pan_started {
            return;
        }
        let current = Vector2::new(x, y);
        if let Some(last_pos) = self.pan_last_position {
            let delta = current - last_pos;
            let radius = self.radius as f32;
            let fovy = camera.get_fovy();
            let scale = (2.0 * radius * (fovy / 2.0).to_radians().tan()) / viewport_height;
            let dx = -delta.x as f32;
            let dy = -delta.y as f32;
            self.pan_offset += camera.right() * (dx * scale) + camera.up_local() * (-dy * scale);
            self.pan_last_position = Some(current);
        } else {
            self.pan_last_position = Some(current);
        }
    }
    pub fn handle_mouse_wheel(&mut self, delta: MouseScrollDelta, _phase: TouchPhase) {
        let scroll_y = match delta {
            MouseScrollDelta::LineDelta(_, y) => y as f64 * 2.0,
            MouseScrollDelta::PixelDelta(pos) => pos.y,
        };
        self.radius -= scroll_y;
        self.radius = self.radius.clamp(3.0, 200.0);
    }
    pub fn update_camera(&self, camera: &mut Camera) {
        let target = Point3::from_vec(self.pan_offset);
        let orbit = spherical_to_cartesian(self.radius, self.yaw, self.pitch).to_vec();
        camera.set_position(target, target + orbit);
    }
}
