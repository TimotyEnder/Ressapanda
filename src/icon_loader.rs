use std::collections::HashMap;

use egui::{TextureHandle, TextureOptions};

const SINGLE_SELECT: &[u8] = include_bytes!("../Assets/Icons/single_select.png");
const AREA_SELECT: &[u8] = include_bytes!("../Assets/Icons/area_select.png");
const EX_AREA_SELECT: &[u8] = include_bytes!("../Assets/Icons/extended_area_select.png");
const LASER_SELECT: &[u8] = include_bytes!("../Assets/Icons/laser_select.png");
const DEL: &[u8] = include_bytes!("../Assets/Icons/del.png");
const ADD: &[u8] = include_bytes!("../Assets/Icons/add.png");
const STYLE: &[u8] = include_bytes!("../Assets/Icons/style.png");
const ADD_VOXEL_GROUP: &[u8] = include_bytes!("../Assets/Icons/add_voxel_group.png");
const DELETE_VOXEL_GROUP: &[u8] = include_bytes!("../Assets/Icons/delete_voxel_group.png");
const MERGE_VOXEL_GROUP: &[u8] = include_bytes!("../Assets/Icons/merge_voxel_group.png");
const MOVE_VOXEL_GROUP_UP: &[u8] = include_bytes!("../Assets/Icons/move_voxel_group_up.png");
const MOVE_VOXEL_GROUP_DOWN: &[u8] = include_bytes!("../Assets/Icons/move_voxel_group_down.png");
const DUPLICATE_VOXEL_GROUP: &[u8] = include_bytes!("../Assets/Icons/duplicate_voxel_group.png");
const VOXEL_GROUP_VISIBLE: &[u8] = include_bytes!("../Assets/Icons/voxel_group_visible.png");
const VOXEL_GROUP_INVISIBLE: &[u8] = include_bytes!("../Assets/Icons/voxel_group_invisible.png");
const RESIZE_VOXEL_GRID: &[u8] = include_bytes!("../Assets/Icons/resize_voxel_grid.png");
const REDO: &[u8] = include_bytes!("../Assets/Icons/redo.png");
const UNDO: &[u8] = include_bytes!("../Assets/Icons/undo.png");
const CUT: &[u8] = include_bytes!("../Assets/Icons/cut.png");

pub struct IconLoader {
    icons: HashMap<String, TextureHandle>,
}
impl IconLoader {
    pub fn new(ctx: egui::Context) -> Self {
        let mut icons = HashMap::new();
        [
            (SINGLE_SELECT, "Single"),
            (AREA_SELECT, "Area"),
            (EX_AREA_SELECT, "Extended"),
            (LASER_SELECT, "Laser"),
            (DEL, "Del"),
            (ADD, "Add"),
            (STYLE, "Style"),
            (ADD_VOXEL_GROUP, "Add_Voxel_Group"),
            (DELETE_VOXEL_GROUP, "Delete_Voxel_Group"),
            (MERGE_VOXEL_GROUP, "Merge_Voxel_Group"),
            (DUPLICATE_VOXEL_GROUP, "Duplicate_Voxel_Group"),
            (MOVE_VOXEL_GROUP_DOWN, "Move_Voxel_Group_Down"),
            (MOVE_VOXEL_GROUP_UP, "Move_Voxel_Group_Up"),
            (VOXEL_GROUP_INVISIBLE, "Voxel_Group_Invisible"),
            (VOXEL_GROUP_VISIBLE, "Voxel_Group_Visible"),
            (RESIZE_VOXEL_GRID, "Resize_Voxel_Grid"),
            (UNDO, "Undo"),
            (REDO, "Redo"),
            (CUT, "Cut"),
        ]
        .iter()
        .for_each(|(icon_const, name)| {
            let image = image::load_from_memory(*icon_const).unwrap().to_rgba8();
            let size = [image.width() as _, image.height() as _];
            let rgba = image.into_raw();
            let texture = ctx.load_texture(
                "icon",
                egui::ColorImage::from_rgba_unmultiplied(size, &rgba),
                TextureOptions::default(),
            );
            icons.insert(String::from(*name), texture);
        });
        Self { icons }
    }
    pub fn get_icon_texture(&self, name: &str) -> Option<&TextureHandle> {
        self.icons.get(name)
    }
}
