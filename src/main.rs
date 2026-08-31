use crate::app::run;

pub mod app;
pub mod camera;
pub mod color;
pub mod state;
pub mod vertex;
pub mod voxel_instance;

fn main() {
    run().unwrap();
}
