use crate::{camera::Camera, tools::tool::Tool, voxel_scene::VoxelScene};

pub trait SelectMode {
    fn mouse_down(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        camera: &Camera,
        scene: &mut VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn Tool>,
    );
    fn mouse_up(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        camera: &Camera,
        scene: &mut VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn Tool>,
    );
    fn temp_draw_on_mouse_hover(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        camera: &Camera,
        scene: &mut VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn Tool>,
    );
}
