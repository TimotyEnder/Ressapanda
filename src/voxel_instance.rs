use cgmath::{Point3, Quaternion, Vector3, Vector4, prelude::*};

use crate::color::VoxelColor;
pub struct VoxelInstance {
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
    color: VoxelColor,
    selected: bool,
    grid_voxel: bool,
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
            selected: false,
            grid_voxel: false,
        }
    }
    pub fn to_raw(&self) -> RawVoxelInstance {
        let info_vec = Vector4::new(
            { if self.selected { 1.0 } else { 0.0 } },
            if self.grid_voxel { 1.0 } else { 0.0 },
            0.0,
            0.0,
        );
        RawVoxelInstance {
            matrix: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
            color: self.color.to_vector_4().into(),
            info_vec: info_vec.into(),
        }
    }
    pub fn point_on_voxel_grid_closest_to_point(
        &self,
        comparison_point: Point3<f32>,
    ) -> Vector3<f32> {
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
        Vector3::new(
            points[points.len() - 1].x,
            points[points.len() - 1].y,
            points[points.len() - 1].z,
        )
    }
    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }
    pub fn get_rotation(&self) -> Quaternion<f32> {
        self.rotation
    }
    pub fn select(&mut self) {
        self.selected = true;
    }
    pub fn unselect(&mut self) {
        self.selected = false;
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawVoxelInstance {
    matrix: [[f32; 4]; 4],
    color: [f32; 4],
    info_vec: [f32; 4], //x selected bool f32 / y grid voxel f32
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
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 20]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
