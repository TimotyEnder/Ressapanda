use cgmath::Vector3;

use crate::{
    brushes::brush::Brush,
    tools::{add::Add, del::Del, fragment::FragmentCut, style::Style},
    voxel_scene::VoxelScene,
};

pub trait Tool {
    fn operate_with_position(
        &mut self,
        operating_position: Vector3<f32>,
        scene: &mut VoxelScene,
        brush: &Brush,
    );
    fn temp_operate_with_position(
        &mut self,
        operating_position: Vector3<f32>,
        scene: &mut VoxelScene,
        brush: &Brush,
    );
    fn name(&self) -> &'static str;
    fn cursor_name(&self) -> &'static str;
    fn tooltip(&self) -> &'static str;
}

pub fn tool_from_name(name: &'static str) -> Option<Box<dyn Tool>> {
    match name {
        "Add" => return Some(Box::new(Add {})),
        "Del" => return Some(Box::new(Del {})),
        "Style" => return Some(Box::new(Style {})),
        "Cut" => return Some(Box::new(FragmentCut {})),
        _ => return None,
    };
}
pub fn tool_tooltip_from_name(name: &str) -> Option<&str> {
    match name {
        "Add" => Some("Add (Shorcut:A)"),
        "Del" => Some("Delete (Shorcut:D)"),
        "Style" => Some("Style/Paint (Shorcut:S)"),
        "Cut" => Some(
            "Fragment cut: add selected  voxels into a fragment that can  be made  into a separate voxel group (Shortcut:F)",
        ),
        _ => return None,
    }
}
