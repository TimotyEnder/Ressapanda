use crate::select_mode::select_mode::SelectMode;

pub struct LineSelectMode {}
impl LineSelectMode {
    pub fn new() -> Self {
        Self {}
    }
}
impl SelectMode for LineSelectMode {
    fn mouse_down(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        modifier_key_status: crate::tools::key_input_manager::ModifierKeysStatus,
        camera: &crate::camera::Camera,
        scene: &mut crate::voxel_scene::VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn crate::tools::tool::Tool>,
        brush: &crate::brushes::brush::Brush,
    ) {
    }

    fn mouse_up(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        modifier_key_status: crate::tools::key_input_manager::ModifierKeysStatus,
        camera: &crate::camera::Camera,
        scene: &mut crate::voxel_scene::VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn crate::tools::tool::Tool>,
        brush: &crate::brushes::brush::Brush,
    ) {
    }

    fn temp_draw_on_mouse_hover(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        modifier_key_status: crate::tools::key_input_manager::ModifierKeysStatus,
        camera: &crate::camera::Camera,
        scene: &mut crate::voxel_scene::VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn crate::tools::tool::Tool>,
        brush: &crate::brushes::brush::Brush,
    ) {
    }

    fn cursor_name(&self) -> &'static str {
        "xln"
    }

    fn name(&self) -> &'static str {
        "Line"
    }

    fn tooltip(&self) -> &'static str {
        "Line Select Mode: drag to form uniform line between two voxels. Selects the voxels in between (Shortcut:T)"
    }
}
