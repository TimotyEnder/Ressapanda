use std::{collections::HashMap, io::SeekFrom};

use winit::keyboard::KeyCode;

use crate::{
    state::State,
    tools::tool::{Tool, tool_from_name},
};

pub struct ToolSelector {
    keycode_to_tool_type: HashMap<KeyCode, &'static str>,
    keycode_to_flag: HashMap<KeyCode, bool>,
}
impl ToolSelector {
    pub fn new() -> Self {
        let mut ret = Self {
            keycode_to_tool_type: HashMap::new(),
            keycode_to_flag: HashMap::new(),
        };
        ret.keycode_to_tool_type.insert(KeyCode::KeyA, "Add");
        ret.keycode_to_flag.insert(KeyCode::KeyA, false);
        ret.keycode_to_tool_type.insert(KeyCode::KeyD, "Del");
        ret.keycode_to_flag.insert(KeyCode::KeyD, false);
        ret
    }
    pub fn tool_selection_inputs(&mut self, key: KeyCode, pressed: bool) -> Option<Box<dyn Tool>> {
        if let Some(name) = self.keycode_to_tool_type.get(&key)
            && let Some(flag) = self.keycode_to_flag.get_mut(&key)
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
}
