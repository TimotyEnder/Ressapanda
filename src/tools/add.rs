use cgmath::Vector3;

use crate::{brushes::brush::Brush, tools::tool::Tool, voxel_scene::VoxelScene};

pub struct Add {}
impl Tool for Add {
    fn operate_with_position(
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

    fn temp_operate_with_position(
        &mut self,
        operating_position: Vector3<f32>,
        scene: &mut VoxelScene,
        brush: &Brush,
    ) {
        scene.add_temporary_voxels(vec![operating_position], &brush.color);
    }

    fn cursor_name(&self) -> &'static str {
        "add"
    }

    fn tooltip(&self) -> &'static str {
        "Add (Shorcut:A)"
    }
}
