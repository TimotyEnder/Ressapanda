use crate::{
    brushes::brush::Brush,
    camera::Camera,
    select_mode::{
        area_select_mode::AreaSelectMode, extended_area_select::ExtendedAreaSelectMode,
        laser_select_mode::LaserSelectMode, single_select_mode::SingleSelectMode,
    },
    tools::{key_input_manager::ModifierKeysStatus, tool::Tool},
    voxel_scene::VoxelScene,
};

pub trait SelectMode {
    fn mouse_down(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        modifier_key_status: ModifierKeysStatus,
        camera: &Camera,
        scene: &mut VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn Tool>,
        brush: &Brush,
    );
    fn mouse_up(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        modifier_key_status: ModifierKeysStatus,
        camera: &Camera,
        scene: &mut VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn Tool>,
        brush: &Brush,
    );
    fn temp_draw_on_mouse_hover(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        modifier_key_status: ModifierKeysStatus,
        camera: &Camera,
        scene: &mut VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn Tool>,
        brush: &Brush,
    );
    fn cursor_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn tooltip(&self) -> &'static str;
}

pub fn select_mode_from_name(name: &'static str) -> Option<Box<dyn SelectMode>> {
    match name {
        "Single" => return Some(Box::new(SingleSelectMode::new())),
        "Area" => return Some(Box::new(AreaSelectMode::new())),
        "Extended" => return Some(Box::new(ExtendedAreaSelectMode::new())),
        "Laser" => return Some(Box::new(LaserSelectMode::new())),
        _ => return None,
    };
}
pub fn select_mode_tooltip_from_name(name: &str) -> Option<&str> {
    match name {
        "Single" => {
            return Some("Single Select Mode (Shortcut:Q). For continuous operation, hold (Shift)");
        }
        "Area" => return Some("Area Select Mode (Shortcut:W)"),
        "Extended" => return Some("Extended Area Select(Shortcut:E)"),
        "Laser" => return Some("Laser Select (Shortcut:r)"),
        _ => return None,
    };
}
