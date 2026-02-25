mod imp {
    use gtk4::glib;
    use gtk4::subclass::prelude::*;
    use libadwaita::subclass::prelude::*;

    use crate::window::MonitorSwitchWindow;

    #[derive(Default)]
    pub struct MonitorSwitchApplication;

    #[glib::object_subclass]
    impl ObjectSubclass for MonitorSwitchApplication {
        const NAME: &'static str = "MonitorSwitchApplication";
        type Type = super::MonitorSwitchApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for MonitorSwitchApplication {}

    impl ApplicationImpl for MonitorSwitchApplication {
        fn activate(&self) {
            let app = self.obj();
            let window = MonitorSwitchWindow::new(&app);
            window.present();
        }
    }

    impl GtkApplicationImpl for MonitorSwitchApplication {}
    impl AdwApplicationImpl for MonitorSwitchApplication {}
}

use gtk4::gio;
use gtk4::glib;
use libadwaita as adw;

glib::wrapper! {
    pub struct MonitorSwitchApplication(ObjectSubclass<imp::MonitorSwitchApplication>)
        @extends adw::Application, gtk4::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl MonitorSwitchApplication {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", crate::APP_ID)
            .property("flags", gio::ApplicationFlags::FLAGS_NONE)
            .build()
    }
}

impl Default for MonitorSwitchApplication {
    fn default() -> Self {
        Self::new()
    }
}

