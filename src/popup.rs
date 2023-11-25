mod wayland;
mod x11;

use gtk4::ApplicationWindow;

use crate::{global_info::GlobalInfo, Backend};

#[derive(Debug, Clone)]
pub struct PopupInfo {
    pub global_info: GlobalInfo,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn setup_popup(backend: Backend, window: &ApplicationWindow, info: &PopupInfo) {
    match backend {
        Backend::X11 => x11::setup_popup(window, info),
        Backend::Wayland => wayland::setup_popup(window, info),
    }
}
