use x11rb::{connection::Connection, protocol::xproto::ConnectionExt};

use super::GlobalInfo;

pub fn get_global_info() -> GlobalInfo {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let root_screen = &conn.setup().roots[screen_num];
    let monitor_width = root_screen.width_in_pixels as _;
    let monitor_height = root_screen.height_in_pixels as _;
    let root_window = root_screen.root;
    let cookie = conn.query_pointer(root_window).unwrap();
    let reply = cookie.reply().unwrap();
    let pointer_x = reply.root_x as _;
    let pointer_y = reply.root_y as _;

    GlobalInfo {
        monitor_width,
        monitor_height,
        pointer_x,
        pointer_y,
    }
}
