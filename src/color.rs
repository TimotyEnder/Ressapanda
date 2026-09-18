use egui::Color32;
#[derive(Clone, Copy)]
pub struct VoxelColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl VoxelColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub fn from_egui_color(color: egui::Color32) -> Self {
        Self {
            r: srgb_to_linear(color.r() as f32 / 255.0),
            g: srgb_to_linear(color.g() as f32 / 255.0),
            b: srgb_to_linear(color.b() as f32 / 255.0),
            a: (color.a() as f32 / 255.0),
        }
    }
    pub fn from_hex(hex: &str) -> anyhow::Result<Self> {
        if hex
            .chars()
            .nth(0)
            .ok_or(anyhow::format_err!("no hex symbol in color"))?
            == '#'
            && (hex.len() == 9 || hex.len() == 7)
        {
            let r255 = u8::from_str_radix(&hex[1..3], 16)?;
            let g255 = u8::from_str_radix(&hex[3..5], 16)?;
            let b255 = u8::from_str_radix(&hex[5..7], 16)?;
            let a255 = if hex.len() == 9 {
                u8::from_str_radix(&hex[7..9], 16).unwrap_or(255)
            } else {
                255
            };
            let r = srgb_to_linear(r255 as f32 / 255.0);
            let g = srgb_to_linear(g255 as f32 / 255.0);
            let b = srgb_to_linear(b255 as f32 / 255.0);
            let a = a255 as f32 / 255.0;
            return Ok(Self { r, g, b, a });
        }
        return Err(anyhow::format_err!("Unable to parse hex color {}", hex));
    }
    pub fn to_vector_4(&self) -> cgmath::Vector4<f32> {
        cgmath::Vector4 {
            x: self.r,
            y: self.g,
            z: self.b,
            w: self.a,
        }
    }
}
impl Default for VoxelColor {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}
pub fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}
pub const ORANGE: Color32 = Color32::from_rgb(219, 135, 88);
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
pub const ACTIVE_BG_STROKE: Color32 = Color32::from_rgb(20, 20, 20);
pub const OPEN_WEAK_BG_FILL: Color32 = Color32::from_rgb(26, 26, 26);
pub const PANEL_FILL: Color32 = Color32::from_rgb(59, 59, 59);
pub const SUB_PANEL_FILL: Color32 = Color32::from_rgb(40, 40, 40);
