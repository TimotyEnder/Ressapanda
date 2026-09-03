use cgmath::Vector3;
use wgpu::Color;

use crate::{brushes::brush::Brush, color::VoxelColor, tools::tool::Tool, voxel_scene::VoxelScene};

pub struct Add {}
impl Tool for Add {
    fn operate_with_voxel_and_intersect(
        &mut self,
        operating_position: Vector3<f32>,
        scene: &mut VoxelScene,
        brush: &Brush,
    ) {
        scene.add_voxel(operating_position, &brush.color);
    }

    fn name(&self) -> &'static str {
        "Add"
    }

    fn temp_operate_with_voxel_and_intersect(
        &mut self,
        operating_position: Vector3<f32>,
        scene: &mut VoxelScene,
        brush: &Brush,
    ) {
        scene.add_temporary_voxels(vec![operating_position], &brush.color);
    }
}
