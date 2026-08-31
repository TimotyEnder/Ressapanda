use anyhow::Ok;

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
            let r = r255 as f32 / 255.0;
            let g = g255 as f32 / 255.0;
            let b = b255 as f32 / 255.0;
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
