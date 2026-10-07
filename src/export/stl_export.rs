use cgmath::Vector3;

use crate::color::VoxelColor;

pub struct StlVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub struct StlFace {
    pub normal: Vector3<f32>,
    pub v1: StlVertex,
    pub v2: StlVertex,
    pub v3: StlVertex,
    pub attribute_byte_count: StlColorInAttributeValue,
}
pub struct StlColorInAttributeValue {
    pub color: VoxelColor,
}
