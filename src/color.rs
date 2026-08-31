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
    pub fn to_vector_4(&self) -> cgmath::Vector4<f32> {
        cgmath::Vector4 {
            x: self.r,
            y: self.g,
            z: self.b,
            w: self.a,
        }
    }
}
