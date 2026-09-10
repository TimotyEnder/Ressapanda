use crate::app::run;

pub mod app;
pub mod brushes;
pub mod camera;
pub mod camera_controller;
pub mod color;
pub mod conversion_utils;
pub mod cursor_loader;
pub mod depth_texture;
pub mod filler;
pub mod icon_loader;
pub mod raycast;
pub mod select_mode;
pub mod state;
pub mod tools;
pub mod ui_data;
pub mod vertex;
pub mod voxel_instance;
pub mod voxel_scene;

fn main() {
    run().unwrap();
}
