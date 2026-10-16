use cgmath::{Point3, Vector3, vec3};

use crate::{
    brushes::brush::Brush,
    camera::Camera,
    select_mode::{
        area_select_mode::AreaSelectMode, extended_area_select::ExtendedAreaSelectMode,
        laser_select_mode::LaserSelectMode, line_select::LineSelectMode,
        single_select_mode::SingleSelectMode,
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
        "Line" => return Some(Box::new(LineSelectMode::new())),
        _ => return None,
    };
}
pub fn select_mode_tooltip_from_name(name: &str) -> Option<&str> {
    match name {
        "Single" => {
            return Some("Single Select Mode (Shortcut:Q). For continuous operation, hold (Shift)");
        }
        "Area" => return Some("Area Select Mode (Shortcut:W)\n(Shift) to clamp to square/cube"),
        "Extended" => {
            return Some(
                "Extended Area Select(Shortcut:E).\n(Shift) to clamp initial selection  to square/cube",
            );
        }
        "Laser" => return Some("Laser Select (Shortcut:r)"),
        "Line" => {
            return Some(
                "Line Select Mode: drag to form uniform line between two voxels. Selects the voxels in between (Shortcut:T)",
            );
        }
        _ => return None,
    };
}
pub fn clamp_voxel_selection_to_maintain_ascpect_ratio_mod_key_conditional(
    raycast_result_opt: Option<(Point3<f32>, Vector3<f32>)>,
    previous_hit: Option<(Point3<f32>, Vector3<f32>)>,
    mod_key_status: ModifierKeysStatus,
) -> Option<(Point3<f32>, Vector3<f32>)> {
    if mod_key_status.shift_modifier {
        if let Some((intersect_pos, voxel_pos)) = raycast_result_opt {
            if let Some((prev_intersect_pos, prev_voxel_pos)) = previous_hit {
                let x_dist = voxel_pos.x - prev_voxel_pos.x;
                let y_dist = voxel_pos.y - prev_voxel_pos.y;
                let z_dist = voxel_pos.z - prev_voxel_pos.z;
                let biggest = (x_dist.abs()).max((y_dist.abs()).max(z_dist.abs()));
                let clamped_pos = vec3(
                    prev_voxel_pos.x + x_dist.signum() * biggest,
                    prev_voxel_pos.y + y_dist.signum() * biggest,
                    prev_voxel_pos.z + z_dist.signum() * biggest,
                );
                let clamped_pos = vec3(
                    if voxel_pos.x == prev_voxel_pos.x {
                        voxel_pos.x
                    } else {
                        prev_voxel_pos.x + x_dist.signum() * biggest
                    },
                    if voxel_pos.y == prev_voxel_pos.y {
                        voxel_pos.y
                    } else {
                        prev_voxel_pos.y + y_dist.signum() * biggest
                    },
                    if voxel_pos.z == prev_voxel_pos.z {
                        voxel_pos.z
                    } else {
                        prev_voxel_pos.z + z_dist.signum() * biggest
                    },
                );
                let intersect_point_diff =
                    voxel_pos - vec3(intersect_pos.x, intersect_pos.y, intersect_pos.z);
                let clamped_intersect = clamped_pos + intersect_point_diff;
                return Some((
                    Point3::new(
                        clamped_intersect.x,
                        clamped_intersect.y,
                        clamped_intersect.z,
                    ),
                    clamped_pos,
                ));
            } else {
                return raycast_result_opt;
            }
        } else {
            return None;
        }
    }
    return raycast_result_opt;
}
