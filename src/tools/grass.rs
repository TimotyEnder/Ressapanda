use crate::tools::tool::Tool;

pub struct Grass {}
impl Grass {
    pub fn new() -> Self {
        Self {}
    }
}
impl Tool for Grass {
    fn operate_with_position(
        &mut self,
        operating_position: cgmath::Vector3<f32>,
        scene: &mut crate::voxel_scene::VoxelScene,
        brush: &crate::brushes::brush::Brush,
    ) {
        scene.grass_operate_on_voxel_pos(operating_position);
    }

    fn temp_operate_with_position(
        &mut self,
        operating_position: cgmath::Vector3<f32>,
        scene: &mut crate::voxel_scene::VoxelScene,
        brush: &crate::brushes::brush::Brush,
    ) {
        scene.select_voxel_at_position_regardless_of_current_voxel_group_selection(
            operating_position,
        );
    }

    fn name(&self) -> &'static str {
        "Grass"
    }

    fn cursor_name(&self) -> &'static str {
        "grss"
    }

    fn tooltip(&self) -> &'static str {
        "Adds grass in a specified area (Shortcut: G)"
    }

    fn update(&mut self) {}
}
