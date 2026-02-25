mod application;
mod autostart;
mod input_row;
mod preferences;
mod window;

use gtk4::gio;
use gtk4::glib;
use gtk4::prelude::*;

pub const APP_ID: &str = "com.github.samneirinck.MonitorSwitch";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("monitor-switch.gresource")
        .expect("Failed to register resources");

    let app = application::MonitorSwitchApplication::new();
    app.run()
}

