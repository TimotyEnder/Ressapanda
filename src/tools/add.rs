use cgmath::Vector3;
use wgpu::Color;

use crate::{color::VoxelColor, tools::tool::Tool, voxel_scene::VoxelScene};

pub struct Add {}
impl Tool for Add {
    fn operate_with_voxel_and_intersect(
        &mut self,
        operating_position: Vector3<f32>,
        scene: &mut VoxelScene,
    ) {
        // if let Some(voxel) = scene.get_voxel_from_position(voxel_position) {
        //     let spawn_point = voxel.point_on_voxel_grid_closest_to_point(intersect_pos);
        //     scene.add_voxel(
        //         Vector3::new(spawn_point.x, spawn_point.y, spawn_point.z),
        //         VoxelColor::default(),
        //     );
        // }
        scene.add_voxel(operating_position, VoxelColor::default());
    }

    fn name(&self) -> &'static str {
        "Add"
    }
}
