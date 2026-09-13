use std::collections::HashMap;

use egui::{TextureHandle, TextureOptions};

const SINGLE_SELECT: &[u8] = include_bytes!("../Assets/Icons/single_select.png");
const AREA_SELECT: &[u8] = include_bytes!("../Assets/Icons/area_select.png");
const EX_AREA_SELECT: &[u8] = include_bytes!("../Assets/Icons/extended_area_select.png");
const DEL: &[u8] = include_bytes!("../Assets/Icons/del.png");
const ADD: &[u8] = include_bytes!("../Assets/Icons/add.png");
const SUBS: &[u8] = include_bytes!("../Assets/Icons/subs.png");

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
            (DEL, "Del"),
            (ADD, "Add"),
            (SUBS, "Subs"),
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
        if let Some(texture) = self.icons.get(name) {
            return Some(texture);
        } else {
            return None;
        }
    }
}
