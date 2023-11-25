use glib::clone;
use gtk4::prelude::*;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::GestureClick;
use gtk4::TextView;
use gtk4::WrapMode;

use crate::popup::setup_popup;
use crate::popup::PopupInfo;
use crate::{global_info::GlobalInfo, Backend};

pub fn create_window(backend: Backend, app: &Application, global_info: &GlobalInfo, text: &str) {
    let window = ApplicationWindow::new(app);
    let css = "
        window {
            padding: 5px;
            border: 1px solid black;
            border-radius: 5px;
        }
        ";
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(css);
    window
        .style_context()
        .add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let text_view = TextView::new();
    let text_buffer = text_view.buffer();
    text_buffer.set_text(text);
    text_view.set_editable(false);
    text_view.set_wrap_mode(WrapMode::Word);
    let css = "
        textview {
            font-size: 18px;
        }
        ";
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(css);
    text_view
        .style_context()
        .add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    window.set_child(Some(&text_view));

    let gesture_click = GestureClick::new();
    gesture_click.connect_released(clone!(@weak window => move |_, _, _, _| {
        window.destroy();
    }));
    window.add_controller(gesture_click);

    setup_popup(backend, &window, &calculate_popup_info(global_info, text));

    window.present();
}

pub fn calculate_popup_info(global_info: &GlobalInfo, text: &str) -> PopupInfo {
    // TODO: Make this more dynamic
    let width = 400;
    let height = (text.lines().count() * 20) as _;

    PopupInfo {
        global_info: global_info.clone(),
        x: global_info.pointer_x,
        y: global_info.pointer_y,
        width,
        height,
    }
}
