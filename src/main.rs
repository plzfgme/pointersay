mod global_info;
mod popup;
mod window;

use std::io::{stdin, Read};

use global_info::get_global_info;
use gtk4::{prelude::*, Application};
use window::create_window;

#[derive(Debug, Clone, Copy)]
enum Backend {
    X11,
    Wayland,
}

fn detect_backend() -> Backend {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        Backend::Wayland
    } else {
        Backend::X11
    }
}

fn main() {
    let text = stdin()
        .bytes()
        .map(|b| b.unwrap() as char)
        .collect::<String>();

    let backend = detect_backend();

    let global_info = get_global_info(backend);

    let application = Application::builder()
        .application_id("com.github.plzfgme.pointersay")
        .build();

    application.connect_activate(move |app| create_window(backend, app, &global_info, &text));

    application.run();
}
