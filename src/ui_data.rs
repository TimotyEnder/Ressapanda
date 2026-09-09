use egui::Color32;

pub struct UIData {
    pub ui_brush_color: Color32,
    pub show_color_picker: bool,
    pub color_hex_input_string: String,
}
impl UIData {
    pub fn new() -> Self {
        Self {
            ui_brush_color: Color32::from_rgb(255, 255, 255),
            show_color_picker: false,
            color_hex_input_string: Color32::from_rgb(255, 255, 255).to_hex(),
        }
    }
}
