use std::f64;

use cgmath::Vector2;
use log::log;
use winit::{
    event::{MouseButton, MouseScrollDelta, TouchPhase},
    keyboard::KeyCode,
};

use crate::{camera::Camera, conversion_utils::spherical_to_cartesian};

pub struct CameraController {
    sensitivity_rad_per_pixel: f64,
    is_rotate_started: bool,
    rotate_last_position: Option<Vector2<f64>>,
    yaw: f64,
    pitch: f64,
    radius: f64,
}
impl CameraController {
    pub fn new(sensitivity: f32, camera: &Camera) -> Self {
        let radius = (camera.get_eye_position().x.powf(2.0)
            + camera.get_eye_position().y.powf(2.0)
            + camera.get_eye_position().z.powf(2.0))
        .sqrt();
        Self {
            sensitivity_rad_per_pixel: sensitivity as f64,
            is_rotate_started: false,
            rotate_last_position: Some(Vector2::new(0.0, 0.0)),
            yaw: 0.0,
            pitch: (camera.get_eye_position().y as f64 / radius as f64).atan(),
            radius: radius as f64,
        }
    }
    pub fn handle_mouse_button(&mut self, button: MouseButton, is_pressed: bool) {
        match button {
            MouseButton::Middle => {
                self.is_rotate_started = is_pressed;
                if !is_pressed {
                    self.rotate_last_position = None;
                }
            }
            _ => {}
        };
    }
    pub fn handle_mouse_position(&mut self, x: f64, y: f64) {
        if !self.is_rotate_started {
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
    pub fn handle_mouse_wheel(&mut self, delta: MouseScrollDelta, _phase: TouchPhase) {
        let scroll_y = match delta {
            MouseScrollDelta::LineDelta(_, y) => y as f64 * 2.0,
            MouseScrollDelta::PixelDelta(pos) => pos.y,
        };
        self.radius -= scroll_y;
        self.radius = self.radius.clamp(3.0, 200.0);
        log::info!("camera radius: {}", self.radius);
    }
    pub fn update_camera(&self, camera: &mut Camera) {
        let eye = spherical_to_cartesian(self.radius, self.yaw, self.pitch);
        camera.set_eye_position(eye);
    }
}
