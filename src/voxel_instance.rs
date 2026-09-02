use cgmath::{Point3, Quaternion, Vector3, prelude::*};

use crate::color::VoxelColor;
pub struct VoxelInstance {
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
    color: VoxelColor,
}
impl VoxelInstance {
    pub fn new(
        position: cgmath::Vector3<f32>,
        rotation: cgmath::Quaternion<f32>,
        color: VoxelColor,
    ) -> Self {
        Self {
            position,
            rotation,
            color,
        }
    }
    pub fn to_raw(&self) -> RawVoxelInstance {
        RawVoxelInstance {
            matrix: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
            color: self.color.to_vector_4().into(),
        }
    }
    pub fn point_on_voxel_grid_closest_to_point(
        &self,
        comparison_point: Point3<f32>,
    ) -> Point3<f32> {
        let mut points = vec![
            Point3 {
                x: self.position.x,
                y: self.position.y + 1.0,
                z: self.position.z,
            },
            Point3 {
                x: self.position.x,
                y: self.position.y - 1.0,
                z: self.position.z,
            },
            Point3 {
                x: self.position.x + 1.0,
                y: self.position.y,
                z: self.position.z,
            },
            Point3 {
                x: self.position.x - 1.0,
                y: self.position.y,
                z: self.position.z,
            },
            Point3 {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z + 1.0,
            },
            Point3 {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z - 1.0,
            },
        ];
        points.sort_by(|x, y| {
            (*y - comparison_point)
                .magnitude()
                .total_cmp(&(*x - comparison_point).magnitude())
        });
        points[points.len() - 1]
    }
    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }
    pub fn get_rotation(&self) -> Quaternion<f32> {
        self.rotation
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawVoxelInstance {
    matrix: [[f32; 4]; 4],
    color: [f32; 4],
}
impl RawVoxelInstance {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<RawVoxelInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,

                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
