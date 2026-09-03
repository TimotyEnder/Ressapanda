use cgmath::Vector3;
use wgpu::Color;

use crate::{color::VoxelColor, tools::tool::Tool, voxel_scene::VoxelScene};

pub struct Del {}
impl Tool for Del {
    fn operate_with_voxel_and_intersect(
        &mut self,
        operating_position: Vector3<f32>,
        scene: &mut VoxelScene,
    ) {
        scene.remove_voxel(operating_position);
    }

    fn name(&self) -> &'static str {
        "Del"
    }
}
