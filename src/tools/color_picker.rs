use egui::Color32;

use crate::{
    color::{VoxelColor, linear_to_srgb, srgb_to_linear},
    tools::tool::Tool,
};

pub struct ColorPicker {
    colors_added: Option<Vec<VoxelColor>>,
}
impl ColorPicker {
    pub fn new() -> Self {
        Self { colors_added: None }
    }
    pub fn compile_into_one_color(&mut self) -> Option<Color32> {
        if let Some(ref colors) = self.colors_added {
            let avg_voxel_color = VoxelColor::new(
                colors.iter().map(|c| c.r).sum::<f32>() / colors.len() as f32,
                colors.iter().map(|c| c.g).sum::<f32>() / colors.len() as f32,
                colors.iter().map(|c| c.b).sum::<f32>() / colors.len() as f32,
                colors.iter().map(|c| c.a).sum::<f32>() / colors.len() as f32,
            );
            return Some(avg_voxel_color.to_egui_color());
        }
        return None;
    }
}
impl Tool for ColorPicker {
    fn operate_with_position(
        &mut self,
        operating_position: cgmath::Vector3<f32>,
        scene: &mut crate::voxel_scene::VoxelScene,
        brush: &crate::brushes::brush::Brush,
    ) {
        if let Some(voxel) =
            scene.get_voxel_from_position_prioritizing_first_selected_set(operating_position)
        {
            self.colors_added
                .get_or_insert_default()
                .push(voxel.get_color());
        }
        scene.set_color_change(self.compile_into_one_color());
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
        "Color_Picker"
    }

    fn cursor_name(&self) -> &'static str {
        "cpick"
    }

    fn tooltip(&self) -> &'static str {
        "Color Picker. Multiple selections will yield an average color value (Shortcut:G)"
    }

    fn update(&mut self) {
        self.colors_added = None;
    }
}
