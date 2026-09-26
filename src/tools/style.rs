use crate::{brushes::brush::Brush, tools::tool::Tool};

pub struct Style {}
impl Tool for Style {
    fn operate_with_position(
        &mut self,
        operating_position: cgmath::Vector3<f32>,
        scene: &mut crate::voxel_scene::VoxelScene,
        brush: &Brush,
    ) {
        if scene.remove_voxel(operating_position) {
            scene.add_voxel(operating_position, &brush.color);
        }
    }

    fn name(&self) -> &'static str {
        "Style"
    }

    fn temp_operate_with_position(
        &mut self,
        operating_position: cgmath::Vector3<f32>,
        scene: &mut crate::voxel_scene::VoxelScene,
        _brush: &Brush,
    ) {
        scene.select_voxel_at_position(operating_position);
    }
    fn cursor_name(&self) -> &'static str {
        "style"
    }
    fn tooltip(&self) -> &'static str {
        "Style/Paint (Shorcut:S)"
    }
}
