use std::cell::Cell;
use std::rc::Rc;

use glib::clone;
use glib::timeout_add_seconds_local;
use glib::ControlFlow;
use gtk4::prelude::*;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Button;
use gtk4::ScrolledWindow;
use gtk4::TextView;
use gtk4::WrapMode;

use crate::popup::setup_popup;
use crate::popup::PopupInfo;
use crate::Settings;
use crate::Timeout;
use crate::{global_info::GlobalInfo, Backend};

pub fn create_window(
    backend: Backend,
    settings: &Settings,
    app: &Application,
    global_info: &GlobalInfo,
    text: &str,
) {
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
    text_view.set_wrap_mode(WrapMode::None);
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
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&text_view));
    scrolled_window.set_vexpand(true);

    let timeout = calculate_timeout(settings, text);

    let close_button = match timeout {
        Some(timeout) => Button::with_label(&format!("Close ({}s)", timeout)),
        None => Button::with_label("Close"),
    };
    close_button.connect_clicked(clone!(@weak window => move |_| {
        window.destroy();
    }));
    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    vbox.append(&scrolled_window);
    vbox.append(&close_button);
    window.set_child(Some(&vbox));

    setup_popup(backend, &window, &calculate_popup_info(global_info, text));

    if let Some(timeout) = timeout {
        let timeout = Rc::new(Cell::new(timeout));
        timeout_add_seconds_local(
            1,
            clone!(@weak window => @default-return ControlFlow::Break, move || {
                timeout.set(timeout.get() - 1);
                close_button.set_label(&format!("Close ({}s)", timeout.get()));
                if timeout.get() == 0 {
                    window.destroy();
                    ControlFlow::Break
                } else {
                    ControlFlow::Continue
                }
            }),
        );
    }

    window.present();
}

pub fn calculate_timeout(settings: &Settings, text: &str) -> Option<u32> {
    match settings.timeout {
        Timeout::None => None,
        // TODO: Use better algorithm
        Timeout::Auto => Some((text.len() as f64 * 0.1) as _),
        Timeout::Fixed(timeout) => Some(timeout),
    }
}

pub fn calculate_popup_info(global_info: &GlobalInfo, text: &str) -> PopupInfo {
    let (top_gap, right_gap, bottom_gap, left_gap) = (
        global_info.pointer_y,
        global_info.monitor_width - global_info.pointer_x,
        global_info.monitor_height - global_info.pointer_y,
        global_info.pointer_x,
    );

    let longest_line_len = text.lines().map(|line| line.len()).max().unwrap_or(0);
    let raw_width = (longest_line_len * 10).min(500).max(200) as _;
    let (rightwards, width) = if raw_width < right_gap {
        (true, raw_width)
    } else if raw_width < left_gap {
        (false, raw_width)
    } else {
        (true, right_gap - 10)
    };
    let num_lines = text.lines().count();
    let raw_height = (num_lines * 30 + 30).min(500).max(60) as _;
    let (upwards, height) = if raw_height < top_gap {
        (true, raw_height)
    } else if raw_height < bottom_gap {
        (false, raw_height)
    } else {
        (true, top_gap - 10)
    };

    PopupInfo {
        global_info: global_info.clone(),
        x: if rightwards {
            global_info.pointer_x
        } else {
            global_info.pointer_x - width
        },
        y: if upwards {
            global_info.pointer_y - height
        } else {
            global_info.pointer_y
        },
        width,
        height,
    }
}
