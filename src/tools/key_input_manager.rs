use std::collections::HashMap;

use cgmath::Point3;
use winit::keyboard::{Key, KeyCode};

use crate::{
    camera::{Camera, CameraLookDirection},
    camera_controller::CameraController,
    select_mode::select_mode::{SelectMode, select_mode_from_name},
    tools::tool::{Tool, tool_from_name},
    voxel_scene::{self, VoxelScene, VoxelSceneDirection},
};
pub enum VoxelGroupAction {
    Create,
    Delete,
    Merge,
    Duplicate,
    MoveUp,
    MoveDown,
    None,
}
pub struct ModifierKeysStatus {
    pub alt_modifier: bool,
    pub shift_modifier: bool,
    pub control_modifier: bool,
}
pub struct KeyInputManager {
    keycode_to_mapping: HashMap<KeyCode, &'static str>,
    keycode_to_flag: HashMap<KeyCode, bool>,
    alt_modifier: bool,
    shift_modifier: bool,
    control_modifier: bool,
}
impl KeyInputManager {
    pub fn new() -> Self {
        let mut ret = Self {
            keycode_to_mapping: HashMap::new(),
            keycode_to_flag: HashMap::new(),
            alt_modifier: false,
            shift_modifier: false,
            control_modifier: false,
        };
        //Tools
        ret.keycode_to_mapping.insert(KeyCode::KeyA, "Add");
        ret.keycode_to_flag.insert(KeyCode::KeyA, false);
        ret.keycode_to_mapping.insert(KeyCode::KeyD, "Del");
        ret.keycode_to_flag.insert(KeyCode::KeyD, false);
        ret.keycode_to_mapping.insert(KeyCode::KeyS, "Subs");
        ret.keycode_to_flag.insert(KeyCode::KeyS, false);
        ret.keycode_to_mapping.insert(KeyCode::KeyF, "Cut");
        ret.keycode_to_flag.insert(KeyCode::KeyF, false);

        //selection modes
        ret.keycode_to_mapping.insert(KeyCode::KeyQ, "Single");
        ret.keycode_to_flag.insert(KeyCode::KeyQ, false);
        ret.keycode_to_mapping.insert(KeyCode::KeyW, "Area");
        ret.keycode_to_flag.insert(KeyCode::KeyW, false);
        ret.keycode_to_mapping.insert(KeyCode::KeyE, "Extended");
        ret.keycode_to_flag.insert(KeyCode::KeyE, false);
        ret.keycode_to_mapping.insert(KeyCode::KeyR, "Laser");
        ret.keycode_to_flag.insert(KeyCode::KeyR, false);
        ret
    }
    pub fn modifier_inputs(&mut self, key: KeyCode, pressed: bool) {
        match key {
            KeyCode::ShiftLeft => {
                self.shift_modifier = pressed;
            }
            KeyCode::AltLeft => {
                self.alt_modifier = pressed;
            }
            KeyCode::ControlLeft => {
                self.control_modifier = pressed;
            }
            _ => {}
        };
    }
    pub fn choose_color_input(&mut self, key: KeyCode, pressed: bool) -> bool {
        return key == KeyCode::KeyC
            && pressed
            && !self.alt_modifier
            && !self.shift_modifier
            && !self.control_modifier;
    }
    pub fn save_input(&mut self, key: KeyCode, pressed: bool) -> bool {
        return self.control_modifier && key == KeyCode::KeyS && pressed;
    }
    pub fn resize_input(&mut self, key: KeyCode, pressed: bool) -> bool {
        return key == KeyCode::KeyZ
            && pressed
            && !self.alt_modifier
            && !self.control_modifier
            && !self.shift_modifier;
    }
    pub fn undo_input(&mut self, key: KeyCode, pressed: bool) -> bool {
        return self.control_modifier && key == KeyCode::KeyZ && pressed;
    }
    pub fn redo_input(&mut self, key: KeyCode, pressed: bool) -> bool {
        return self.control_modifier && key == KeyCode::KeyY && pressed;
    }
    pub fn voxel_group_action_inputs(&mut self, key: KeyCode, pressed: bool) -> VoxelGroupAction {
        if self.shift_modifier && pressed {
            match key {
                KeyCode::Digit1 => return VoxelGroupAction::Create,
                KeyCode::Digit2 => return VoxelGroupAction::Delete,
                KeyCode::Digit3 => return VoxelGroupAction::Merge,
                KeyCode::Digit4 => return VoxelGroupAction::Duplicate,
                KeyCode::Digit5 => return VoxelGroupAction::MoveUp,
                KeyCode::Digit6 => return VoxelGroupAction::MoveDown,
                _ => return VoxelGroupAction::None,
            }
        }
        return VoxelGroupAction::None;
    }
    pub fn tool_selection_inputs(&mut self, key: KeyCode, pressed: bool) -> Option<Box<dyn Tool>> {
        if let Some(name) = self.keycode_to_mapping.get(&key)
            && let Some(flag) = self.keycode_to_flag.get_mut(&key)
            && !self.alt_modifier
            && !self.shift_modifier
            && !self.control_modifier
        {
            if !*flag && pressed {
                if let Some(tool) = tool_from_name(name) {
                    return Some(tool);
                }
            } else if !pressed {
                *flag = false;
            }
        }
        return None;
    }
    pub fn select_mode_selection_inputs(
        &mut self,
        key: KeyCode,
        pressed: bool,
    ) -> Option<Box<dyn SelectMode>> {
        if let Some(name) = self.keycode_to_mapping.get(&key)
            && let Some(flag) = self.keycode_to_flag.get_mut(&key)
            && !self.alt_modifier
            && !self.shift_modifier
            && !self.control_modifier
        {
            if !*flag && pressed {
                if let Some(select_mode) = select_mode_from_name(name) {
                    return Some(select_mode);
                }
            } else if !pressed {
                *flag = false;
            }
        }
        return None;
    }
    pub fn camera_preset_positions_inputs(
        &mut self,
        key: KeyCode,
        pressed: bool,
        camera: &mut Camera,
        camera_controller: &mut CameraController,
    ) {
        if pressed && !self.alt_modifier && !self.control_modifier && !self.shift_modifier {
            let target = Point3::new(0.0, 0.0, 0.0);
            let look_direction = match key {
                KeyCode::Digit1 => {
                    // x (view along +X)
                    CameraLookDirection::Xplus
                }
                KeyCode::Digit2 => {
                    // z (view along +Z)
                    CameraLookDirection::Zplus
                }
                KeyCode::Digit3 => {
                    // x backwards (view along -X)
                    CameraLookDirection::Xminus
                }
                KeyCode::Digit4 => {
                    // z backwards (view along -Z)
                    CameraLookDirection::Zminus
                }
                KeyCode::Digit5 => {
                    // straight up (view straight down)
                    CameraLookDirection::Down
                }
                _ => CameraLookDirection::None,
            };
            if look_direction != CameraLookDirection::None {
                let eye = look_direction.eye_position();
                camera_controller.set_preset_orbit(eye);
                camera.set_position(target, target + eye);
            }
        }
    }
    pub fn move_commands_inputs(
        &mut self,
        key: KeyCode,
        pressed: bool,
        scene: &mut VoxelScene,
        camera: &Camera,
    ) {
        if self.alt_modifier {
            self.center_input(key, pressed, scene, camera);
            self.move_inputs(key, pressed, scene, camera);
        } else if self.shift_modifier {
            self.rotate_inputs(key, pressed, scene, camera);
        }
    }
    fn center_input(&self, key: KeyCode, pressed: bool, scene: &mut VoxelScene, _camera: &Camera) {
        if key == KeyCode::KeyC && pressed {
            scene.reposition_to_calculated_center();
        }
    }
    fn move_inputs(
        &mut self,
        key: KeyCode,
        pressed: bool,
        scene: &mut VoxelScene,
        camera: &Camera,
    ) {
        let direction = match key {
            KeyCode::KeyW => VoxelSceneDirection::Forwards,
            KeyCode::KeyS => VoxelSceneDirection::Backwards,
            KeyCode::KeyA => VoxelSceneDirection::Left,
            KeyCode::KeyD => VoxelSceneDirection::Right,
            KeyCode::KeyQ => VoxelSceneDirection::UpLeftSteer,
            KeyCode::KeyE => VoxelSceneDirection::DownRightSteer,
            _ => VoxelSceneDirection::None,
        };
        if pressed && direction != VoxelSceneDirection::None {
            let move_vec = direction.move_vector(camera);
            scene.move_by_vector(move_vec);
        }
    }
    fn rotate_inputs(
        &mut self,
        key: KeyCode,
        pressed: bool,
        scene: &mut VoxelScene,
        camera: &Camera,
    ) {
        let direction = match key {
            KeyCode::KeyW => VoxelSceneDirection::Forwards,
            KeyCode::KeyS => VoxelSceneDirection::Backwards,
            KeyCode::KeyA => VoxelSceneDirection::Left,
            KeyCode::KeyD => VoxelSceneDirection::Right,
            KeyCode::KeyQ => VoxelSceneDirection::UpLeftSteer,
            KeyCode::KeyE => VoxelSceneDirection::DownRightSteer,
            _ => VoxelSceneDirection::None,
        };
        if pressed && direction != VoxelSceneDirection::None {
            let (axis, deg) = direction.rotate_parameters(camera);
            scene.rotate_around_center(axis, deg);
        }
    }
    pub fn get_modifier_keys_status(&self) -> ModifierKeysStatus {
        ModifierKeysStatus {
            alt_modifier: self.alt_modifier,
            shift_modifier: self.shift_modifier,
            control_modifier: self.control_modifier,
        }
    }
}
