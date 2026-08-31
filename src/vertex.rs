#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}
impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        return wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        };
    }
}
pub const CUBE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, -0.5, -0.5],
    }, // 0 - back-bottom-left
    Vertex {
        position: [0.5, -0.5, -0.5],
    }, // 1 - back-bottom-right
    Vertex {
        position: [0.5, 0.5, -0.5],
    }, // 2 - back-top-right
    Vertex {
        position: [-0.5, 0.5, -0.5],
    }, // 3 - back-top-left
    Vertex {
        position: [-0.5, -0.5, 0.5],
    }, // 4 - front-bottom-left
    Vertex {
        position: [0.5, -0.5, 0.5],
    }, // 5 - front-bottom-right
    Vertex {
        position: [0.5, 0.5, 0.5],
    }, // 6 - front-top-right
    Vertex {
        position: [-0.5, 0.5, 0.5],
    }, // 7 - front-top-left
];

pub const CUBE_INDICES: &[u16] = &[
    0, 3, 2, 0, 2, 1, // back
    4, 5, 6, 4, 6, 7, // front
    3, 7, 6, 3, 6, 2, // top
    0, 1, 5, 0, 5, 4, // bottom
    1, 2, 6, 1, 6, 5, // right
    0, 4, 7, 0, 7, 3, // left
];
pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
    }, // E
];

pub const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, /* padding */ 0];
