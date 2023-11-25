use gdk4_x11::X11Surface;
use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use x11rb::connect;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::AtomEnum;
use x11rb::protocol::xproto::ChangeWindowAttributesAux;
use x11rb::protocol::xproto::ConfigureWindowAux;
use x11rb::protocol::xproto::ConnectionExt;
use x11rb::protocol::xproto::PropMode;
use x11rb::wrapper::ConnectionExt as _;

use super::PopupInfo;

pub fn setup_popup(window: &ApplicationWindow, info: &PopupInfo) {
    gtk4::prelude::WidgetExt::realize(window);

    window.set_default_size(info.width as _, info.height as _);

    let surface = window.surface();
    let x11_surface = surface.downcast_ref::<X11Surface>().unwrap();
    let x11_window = x11_surface.xid() as x11rb::protocol::xproto::Window;

    let (conn, _) = connect(None).unwrap();

    #[allow(non_snake_case)]
    let _NET_WM_WINDOW_TYPE = conn
        .intern_atom(true, b"_NET_WM_WINDOW_TYPE")
        .unwrap()
        .reply()
        .unwrap()
        .atom;
    #[allow(non_snake_case)]
    let _NET_WM_WINDOW_TYPE_DOCK = conn
        .intern_atom(true, b"_NET_WM_WINDOW_TYPE_DOCK")
        .unwrap()
        .reply()
        .unwrap()
        .atom;
    conn.change_property32(
        PropMode::REPLACE,
        x11_window,
        _NET_WM_WINDOW_TYPE,
        AtomEnum::ATOM,
        &[_NET_WM_WINDOW_TYPE_DOCK],
    )
    .unwrap()
    .check()
    .unwrap();

    #[allow(non_snake_case)]
    let _NET_WM_STATE = conn
        .intern_atom(true, b"_NET_WM_STATE")
        .unwrap()
        .reply()
        .unwrap()
        .atom;
    #[allow(non_snake_case)]
    let _NET_WM_STATE_ABOVE = conn
        .intern_atom(true, b"_NET_WM_STATE_ABOVE")
        .unwrap()
        .reply()
        .unwrap()
        .atom;
    conn.change_property32(
        PropMode::REPLACE,
        x11_window,
        _NET_WM_STATE,
        AtomEnum::ATOM,
        &[_NET_WM_STATE_ABOVE],
    )
    .unwrap()
    .check()
    .unwrap();

    let change_window_attributes_aux = ChangeWindowAttributesAux::default().override_redirect(1);
    conn.change_window_attributes(x11_window, &change_window_attributes_aux)
        .unwrap()
        .check()
        .unwrap();

    let configure_window_aux = ConfigureWindowAux::default()
        .x(Some(info.x as _))
        .y(Some(info.y as _));
    conn.configure_window(x11_window, &configure_window_aux)
        .unwrap()
        .check()
        .unwrap();
    conn.flush().unwrap();
}
