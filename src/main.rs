use crate::app::run;

pub mod app;
pub mod camera;
pub mod camera_controller;
pub mod color;
pub mod conversion_utils;
pub mod depth_texture;
pub mod state;
pub mod vertex;
pub mod voxel_instance;
pub mod voxel_scene;

fn main() {
    run().unwrap();
}
