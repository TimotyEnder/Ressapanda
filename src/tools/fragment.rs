use crate::tools::tool::Tool;

pub struct FragmentCut {}
impl Tool for FragmentCut {
    fn operate_with_position(
        &mut self,
        operating_position: cgmath::Vector3<f32>,
        scene: &mut crate::voxel_scene::VoxelScene,
        brush: &crate::brushes::brush::Brush,
    ) {
        scene.cut_operate_on_voxel_pos(operating_position);
    }

    fn temp_operate_with_position(
        &mut self,
        operating_position: cgmath::Vector3<f32>,
        scene: &mut crate::voxel_scene::VoxelScene,
        brush: &crate::brushes::brush::Brush,
    ) {
        scene.select_voxel_at_position(operating_position);
    }

    fn name(&self) -> &'static str {
        "Cut"
    }

    fn cursor_name(&self) -> &'static str {
        "cut"
    }

    fn tooltip(&self) -> &'static str {
        "Fragment cut: add selected  voxels into a fragment that can  be separated into a voxel group (Shortcut:F)"
    }
}
