use crate::{
    camera::Camera,
    select_mode::{area_select_mode::AreaSelectMode, single_select_mode::SingleSelectMode},
    tools::tool::Tool,
    voxel_scene::VoxelScene,
};

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

pub fn select_mode_from_name(name: &'static str) -> Option<Box<dyn SelectMode>> {
    match name {
        "Single" => return Some(Box::new(SingleSelectMode::new())),
        "Area" => return Some(Box::new(AreaSelectMode::new())),
        _ => return None,
    };
}
