use std::collections::HashMap;

use winit::window::{Cursor, CustomCursor, Window};

use crate::{select_mode::select_mode::SelectMode, tools::tool::Tool};

const X1_ADD: &[u8] = include_bytes!("../Assets/Cursors/x1add.png");
const X1_STYLE: &[u8] = include_bytes!("../Assets/Cursors/x1style.png");
const X1_DEL: &[u8] = include_bytes!("../Assets/Cursors/x1del.png");
const X1_CUT: &[u8] = include_bytes!("../Assets/Cursors/x1cut.png");
const X1_CPICK: &[u8] = include_bytes!("../Assets/Cursors/x1cpick.png");
const XN_ADD: &[u8] = include_bytes!("../Assets/Cursors/xnadd.png");
const XN_STYLE: &[u8] = include_bytes!("../Assets/Cursors/xnstyle.png");
const XN_DEL: &[u8] = include_bytes!("../Assets/Cursors/xndel.png");
const XN_CUT: &[u8] = include_bytes!("../Assets/Cursors/xncut.png");
const XN_CPICK: &[u8] = include_bytes!("../Assets/Cursors/xncpick.png");
const XE_ADD: &[u8] = include_bytes!("../Assets/Cursors/xeadd.png");
const XE_STYLE: &[u8] = include_bytes!("../Assets/Cursors/xestyle.png");
const XE_DEL: &[u8] = include_bytes!("../Assets/Cursors/xedel.png");
const XE_CUT: &[u8] = include_bytes!("../Assets/Cursors/xecut.png");
const XE_CPICK: &[u8] = include_bytes!("../Assets/Cursors/xecpick.png");
const XEE_ADD: &[u8] = include_bytes!("../Assets/Cursors/xeeadd.png");
const XEE_STYLE: &[u8] = include_bytes!("../Assets/Cursors/xeestyle.png");
const XEE_DEL: &[u8] = include_bytes!("../Assets/Cursors/xeedel.png");
const XEE_CUT: &[u8] = include_bytes!("../Assets/Cursors/xeecut.png");
const XEE_CPICK: &[u8] = include_bytes!("../Assets/Cursors/xeecpick.png");
const XL_ADD: &[u8] = include_bytes!("../Assets/Cursors/xladd.png");
const XL_STYLE: &[u8] = include_bytes!("../Assets/Cursors/xlstyle.png");
const XL_DEL: &[u8] = include_bytes!("../Assets/Cursors/xldel.png");
const XL_CUT: &[u8] = include_bytes!("../Assets/Cursors/xlcut.png");
const XL_CPICK: &[u8] = include_bytes!("../Assets/Cursors/xlcpick.png");
const XLN_ADD: &[u8] = include_bytes!("../Assets/Cursors/xlnadd.png");
const XLN_STYLE: &[u8] = include_bytes!("../Assets/Cursors/xlnstyle.png");
const XLN_DEL: &[u8] = include_bytes!("../Assets/Cursors/xlndel.png");
const XLN_CUT: &[u8] = include_bytes!("../Assets/Cursors/xlncut.png");
const XLN_CPICK: &[u8] = include_bytes!("../Assets/Cursors/xlncpick.png");

pub struct CursorLoader {
    cursors: HashMap<String, CustomCursor>,
}
impl CursorLoader {
    pub fn new(event_loop: &winit::event_loop::ActiveEventLoop) -> Self {
        let mut cursors = HashMap::new();
        [
            (X1_ADD, "x1_add"),
            (X1_DEL, "x1_del"),
            (X1_STYLE, "x1_style"),
            (X1_CUT, "x1_cut"),
            (X1_CPICK, "x1_cpick"),
            (XN_ADD, "xn_add"),
            (XN_DEL, "xn_del"),
            (XN_STYLE, "xn_style"),
            (XN_CUT, "xn_cut"),
            (XN_CPICK, "xn_cpick"),
            (XE_ADD, "xe_add"),
            (XE_DEL, "xe_del"),
            (XE_STYLE, "xe_style"),
            (XE_CUT, "xe_cut"),
            (XE_CPICK, "xe_cpick"),
            (XEE_ADD, "xee_add"),
            (XEE_DEL, "xee_del"),
            (XEE_STYLE, "xee_style"),
            (XEE_CUT, "xee_cut"),
            (XEE_CPICK, "xee_cpick"),
            (XL_ADD, "xl_add"),
            (XL_DEL, "xl_del"),
            (XL_STYLE, "xl_style"),
            (XL_CUT, "xl_cut"),
            (XL_CPICK, "xl_cpick"),
            (XLN_ADD, "xln_add"),
            (XLN_DEL, "xln_del"),
            (XLN_STYLE, "xln_style"),
            (XLN_CUT, "xln_cut"),
            (XLN_CPICK, "xln_cpick"),
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
        window: &Window,
        current_select_mode: &Box<dyn SelectMode>,
        current_tool: &Box<dyn Tool>,
    ) {
        let name = format!(
            "{}_{}",
            current_select_mode.cursor_name(),
            current_tool.cursor_name()
        );
        if let Some(cursor) = self.cursors.get(&name) {
            window.set_cursor(Cursor::Custom(cursor.clone()));
        }
    }
}
