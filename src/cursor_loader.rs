use std::{collections::HashMap, sync::Arc};

use winit::{
    event_loop,
    window::{Cursor, CustomCursor, CustomCursorSource, Window},
};

use crate::{select_mode::select_mode::SelectMode, tools::tool::Tool};

const X1_ADD: &[u8] = include_bytes!("../Assets/Cursors/x1add.png");
const X1_SUBS: &[u8] = include_bytes!("../Assets/Cursors/x1subs.png");
const X1_DEL: &[u8] = include_bytes!("../Assets/Cursors/x1del.png");
const XN_ADD: &[u8] = include_bytes!("../Assets/Cursors/xnadd.png");
const XN_SUBS: &[u8] = include_bytes!("../Assets/Cursors/xnsubs.png");
const XN_DEL: &[u8] = include_bytes!("../Assets/Cursors/xndel.png");

pub struct CursorLoader {
    cursors: HashMap<String, CustomCursor>,
}
impl CursorLoader {
    pub fn new(event_loop: &winit::event_loop::ActiveEventLoop) -> Self {
        let mut cursors = HashMap::new();
        [
            (X1_ADD, "x1_add"),
            (X1_DEL, "x1_del"),
            (X1_SUBS, "x1_subs"),
            (XN_ADD, "xn_add"),
            (XN_DEL, "xn_del"),
            (XN_SUBS, "xn_subs"),
        ]
        .iter()
        .for_each(|(cursor_const, name)| {
            let img = image::load_from_memory(*cursor_const).unwrap().to_rgba8();
            let source = CustomCursor::from_rgba(img.into_raw(), 64, 64, 32, 32).unwrap();
            let cursor = event_loop.create_custom_cursor(source);
            cursors.insert(String::from(*name), cursor);
        });
        Self { cursors }
    }
    pub fn change_cursor(
        &self,
        window: Arc<Window>,
        current_select_mode: &Box<dyn SelectMode>,
        current_tool: &Box<dyn Tool>,
    ) {
        let name = format!(
            "{}{}",
            current_select_mode.cursor_name(),
            current_tool.cursor_name()
        );
        if let Some(cursor) = self.cursors.get(&name) {
            window.set_cursor(Cursor::Custom(cursor.clone()));
        }
    }
}
