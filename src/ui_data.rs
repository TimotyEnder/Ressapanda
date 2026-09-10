use egui::Color32;

use crate::icon_loader::IconLoader;

pub struct UIData {
    pub ui_brush_color: Color32,
    pub show_color_picker: bool,
    pub color_hex_input_string: String,
    pub icon_loader: IconLoader,
}
impl UIData {
    pub fn new(ctx: egui::Context) -> Self {
        Self {
            ui_brush_color: Color32::from_rgb(255, 255, 255),
            show_color_picker: false,
            color_hex_input_string: Color32::from_rgb(255, 255, 255).to_hex(),
            icon_loader: IconLoader::new(ctx),
        }
    }
}
