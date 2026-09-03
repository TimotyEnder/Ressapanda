use cgmath::Vector3;
use wgpu::Color;

use crate::{color::VoxelColor, tools::tool::Tool, voxel_scene::VoxelScene};

pub struct Del {}
impl Tool for Del {
    fn operate_with_voxel_and_intersect(
        &mut self,
        voxel_position: Vector3<f32>,
        intersect_pos: cgmath::Point3<f32>,
        scene: &mut VoxelScene,
    ) {
        scene.remove_voxel(voxel_position);
    }
}
