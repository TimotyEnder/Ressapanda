use crate::{brushes::brush::Brush, color::VoxelColor, tools::tool::Tool};

pub struct Subs {}
impl Tool for Subs {
    fn operate_with_voxel_and_intersect(
        &mut self,
        operating_position: cgmath::Vector3<f32>,
        scene: &mut crate::voxel_scene::VoxelScene,
        brush: &Brush,
    ) {
        scene.remove_voxel(operating_position);
        scene.add_voxel(operating_position, &brush.color);
    }

    fn name(&self) -> &'static str {
        "Subs"
    }

    fn temp_operate_with_voxel_and_intersect(
        &mut self,
        operating_position: cgmath::Vector3<f32>,
        scene: &mut crate::voxel_scene::VoxelScene,
        brush: &Brush,
    ) {
        scene.select_voxel_at_position(operating_position);
    }
    fn cursor_name(&self) -> &'static str {
        "subs"
    }
    fn tooltip(&self) -> &'static str {
        "Substitute/Paint (Shorcut:S)"
    }
}
