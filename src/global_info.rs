use crate::Backend;

mod wayland;
mod x11;

#[derive(Debug, Clone)]
pub struct GlobalInfo {
    pub monitor_width: u32,
    pub monitor_height: u32,
    pub pointer_x: u32,
    pub pointer_y: u32,
}

pub fn get_global_info(backend: Backend) -> GlobalInfo {
    match backend {
        Backend::X11 => x11::get_global_info(),
        Backend::Wayland => wayland::get_global_info(),
    }
}
