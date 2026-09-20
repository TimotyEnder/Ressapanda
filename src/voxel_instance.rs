use cgmath::{Point3, Vector3, Vector4, prelude::*, vec3};

use crate::{
    change::change::VoxelSnapshot, color::VoxelColor, save::SavedVoxel,
    voxel_scene::VoxelScenePosition,
};
#[derive(Clone, Copy)]
pub struct VoxelInstance {
    position: cgmath::Vector3<f32>,
    color: VoxelColor,
    selected: bool,
    grid_voxel: bool,
}
impl VoxelInstance {
    pub fn to_saved(&self) -> SavedVoxel {
        let position = VoxelScenePosition::from_voxel_position(self.position);
        SavedVoxel {
            x: position.x,
            y: position.y,
            z: position.z,
            r: self.color.r,
            g: self.color.g,
            b: self.color.b,
            a: self.color.a,
        }
    }
    pub fn to_snapshot(&self) -> VoxelSnapshot {
        let position = VoxelScenePosition::from_voxel_position(self.position);
        VoxelSnapshot {
            x: position.x,
            y: position.y,
            z: position.z,
            r: self.color.r,
            g: self.color.g,
            b: self.color.b,
            a: self.color.a,
        }
    }
    pub fn from_saved(save: SavedVoxel) -> Self {
        Self {
            position: vec3(save.x as f32, save.y as f32, save.z as f32),
            color: VoxelColor::new(save.r, save.g, save.b, save.a),
            selected: false,
            grid_voxel: false,
        }
    }
    pub fn from_snapshot(snap: &VoxelSnapshot) -> Self {
        Self {
            position: vec3(snap.x as f32, snap.y as f32, snap.z as f32),
            color: VoxelColor::new(snap.r, snap.g, snap.b, snap.a),
            selected: false,
            grid_voxel: false,
        }
    }
    pub fn new(position: cgmath::Vector3<f32>, color: VoxelColor) -> Self {
        Self {
            position,
            color,
            selected: false,
            grid_voxel: false,
        }
    }
    pub fn new_grid_voxel(position: cgmath::Vector3<f32>, color: VoxelColor) -> Self {
        Self {
            position,
            color,
            selected: false,
            grid_voxel: true,
        }
    }
    pub fn to_raw(&self) -> RawVoxelInstance {
        let info_vec = Vector4::new(
            if self.selected { 1.0 } else { 0.0 },
            if self.grid_voxel { 1.0 } else { 0.0 },
            0.0,
            0.0,
        );
        RawVoxelInstance {
            matrix: (cgmath::Matrix4::from_translation(self.position)).into(),
            color: self.color.to_vector_4().into(),
            info_vec: info_vec.into(),
        }
    }
    pub fn point_on_voxel_grid_closest_to_point(
        &self,
        comparison_point: Point3<f32>,
    ) -> Vector3<f32> {
        let mut points = [
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
    pub fn set_position(&mut self, position: cgmath::Vector3<f32>) {
        self.position = position;
    }
    pub fn move_position_by_vector(&mut self, vector: Vector3<f32>) {
        self.position += vector;
    }
    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn select(&mut self) {
        self.selected = true;
    }
    pub fn unselect(&mut self) {
        self.selected = false;
    }
    pub fn is_grid(&self) -> bool {
        self.grid_voxel
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawVoxelInstance {
    matrix: [[f32; 4]; 4],
    color: [f32; 4],
    info_vec: [f32; 4], //x selected bool f32 / y grid voxel bool f32
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
