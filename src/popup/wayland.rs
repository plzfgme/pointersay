use gtk4::ApplicationWindow;
use gtk4_layer_shell::Edge;
use gtk4_layer_shell::KeyboardMode;
use gtk4_layer_shell::Layer;
use gtk4_layer_shell::LayerShell;

use super::PopupInfo;

pub fn setup_popup(window: &ApplicationWindow, info: &PopupInfo) {
    window.init_layer_shell();
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Right, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Bottom, true);
    window.set_keyboard_mode(KeyboardMode::None);
    window.set_layer(Layer::Overlay);

    let margins = calculate_margins(info);
    window.set_margin(Edge::Top, margins.top);
    window.set_margin(Edge::Right, margins.right);
    window.set_margin(Edge::Bottom, margins.bottom);
    window.set_margin(Edge::Left, margins.left);
}

#[derive(Debug)]
struct Margins {
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
}

fn calculate_margins(info: &PopupInfo) -> Margins {
    Margins {
        top: info.y as _,
        right: info.global_info.monitor_width as i32 - info.x as i32 - info.width as i32,
        bottom: info.global_info.monitor_height as i32 - info.y as i32 - info.height as i32,
        left: info.x as _,
    }
}
