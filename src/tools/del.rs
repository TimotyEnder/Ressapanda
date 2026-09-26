use cgmath::Vector3;

use crate::{brushes::brush::Brush, tools::tool::Tool, voxel_scene::VoxelScene};

pub struct Del {}
impl Tool for Del {
    fn operate_with_position(
        &mut self,
        operating_position: Vector3<f32>,
        scene: &mut VoxelScene,
        _brush: &Brush,
    ) {
        scene.remove_voxel(operating_position);
    }

    fn name(&self) -> &'static str {
        "Del"
    }

    fn temp_operate_with_position(
        &mut self,
        operating_position: Vector3<f32>,
        scene: &mut VoxelScene,
        _brush: &Brush,
    ) {
        scene.select_voxel_at_position(operating_position);
    }
    fn cursor_name(&self) -> &'static str {
        "del"
    }
    fn tooltip(&self) -> &'static str {
        "Delete (Shorcut:D)"
    }

    fn update(&mut self) {}
}
