mod autostart;
mod preferences;
mod window;

use gtk4::prelude::*;
use gtk4::{gio, glib};
use libadwaita as adw;

const APP_ID: &str = "com.github.samneirinck.MonitorSwitch";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::FLAGS_NONE)
        .build();

    app.connect_activate(|app| {
        let win = window::MainWindow::new(app);
        win.present();
    });

    app.run()
}

