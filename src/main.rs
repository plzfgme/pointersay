mod global_info;
mod popup;
mod window;

use std::io::{stdin, Read};

use clap::Parser;
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(
        short,
        long,
        help = "Close the window after a timeout (none, auto, or number of seconds like 5)",
        default_value = "auto"
    )]
    timeout: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Timeout {
    None,
    Auto,
    Fixed(u32),
}

#[derive(Debug)]
pub struct Settings {
    pub timeout: Timeout,
}

impl Settings {
    pub fn from_args(args: &Args) -> Self {
        let timeout = match args.timeout.as_str() {
            "none" => Timeout::None,
            "auto" => Timeout::Auto,
            _ => {
                let timeout = args.timeout.parse().expect("Invalid timeout");
                if timeout == 0 {
                    panic!("Timeout must be greater than 0");
                }
                Timeout::Fixed(timeout)
            }
        };

        Self { timeout }
    }
}

fn main() {
    let args = Args::parse();
    let settings = Settings::from_args(&args);

    let mut text = String::new();
    stdin().read_to_string(&mut text).unwrap();

    let backend = detect_backend();

    let global_info = get_global_info(backend);

    let application = Application::builder()
        .application_id("com.github.plzfgme.pointersay")
        .build();

    application
        .connect_activate(move |app| create_window(backend, &settings, app, &global_info, &text));

    application.run_with_args(&Vec::<String>::new());
}
