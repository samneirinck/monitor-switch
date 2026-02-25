mod imp {
    use gtk4::glib;
    use gtk4::subclass::prelude::*;
    use gtk4::{Button, CheckButton, CompositeTemplate, ListBox, TemplateChild};
    use libadwaita::subclass::prelude::*;
    use std::cell::RefCell;

    use monitor_core::Config;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/github/samneirinck/MonitorSwitch/window.ui")]
    pub struct MonitorSwitchWindow {
        #[template_child]
        pub list_box: TemplateChild<ListBox>,
        #[template_child]
        pub prefs_button: TemplateChild<Button>,
        #[template_child]
        pub refresh_button: TemplateChild<Button>,
        #[template_child]
        pub autostart_check: TemplateChild<CheckButton>,

        pub config: RefCell<Config>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MonitorSwitchWindow {
        const NAME: &'static str = "MonitorSwitchWindow";
        type Type = super::MonitorSwitchWindow;
        type ParentType = libadwaita::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MonitorSwitchWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.config.replace(Config::load());
        }
    }

    impl WidgetImpl for MonitorSwitchWindow {}
    impl WindowImpl for MonitorSwitchWindow {}
    impl ApplicationWindowImpl for MonitorSwitchWindow {}
    impl AdwApplicationWindowImpl for MonitorSwitchWindow {}
}

use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::ObjectSubclassIsExt;
use gtk4::{Align, Label, ListBoxRow, Orientation, Separator};
use libadwaita as adw;
use monitor_core::{Config, InputSource, Monitor};

use crate::autostart::Autostart;
use crate::input_row::MonitorSwitchInputRow;
use crate::preferences::PreferencesWindow;

glib::wrapper! {
    pub struct MonitorSwitchWindow(ObjectSubclass<imp::MonitorSwitchWindow>)
        @extends adw::ApplicationWindow, gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Native,
                    gtk4::Root, gtk4::ShortcutManager;
}

struct MonitorData {
    id: String,
    name: String,
    index: usize,
    current_input: Option<InputSource>,
    available_inputs: Vec<InputSource>,
}

impl MonitorSwitchWindow {
    pub fn new(app: &adw::Application) -> Self {
        let window: Self = glib::Object::builder()
            .property("application", app)
            .build();

        window.setup_callbacks();
        window.populate_list();

        window
    }

    fn setup_callbacks(&self) {
        let imp = self.imp();

        imp.autostart_check.set_active(Autostart::is_enabled());
        imp.autostart_check.connect_toggled(|check| {
            if check.is_active() {
                Autostart::enable();
            } else {
                Autostart::disable();
            }
        });

        let window = self.clone();
        imp.prefs_button.connect_clicked(move |_| {
            let prefs = PreferencesWindow::new(&window);
            prefs.present();
        });

        let window = self.clone();
        imp.refresh_button.connect_clicked(move |_| {
            window.refresh();
        });

        let window = self.clone();
        imp.list_box.connect_row_activated(move |_, row| {
            if let Some(input_row) = row.downcast_ref::<MonitorSwitchInputRow>() {
                window.switch_input(input_row.monitor_index(), input_row.input());
            }
        });
    }

    pub fn refresh(&self) {
        self.imp().config.replace(Config::load());
        self.clear_list();
        self.populate_list();
    }

    fn clear_list(&self) {
        let list_box = &self.imp().list_box;
        while let Some(child) = list_box.first_child() {
            list_box.remove(&child);
        }
    }

    fn populate_list(&self) {
        let imp = self.imp();
        let config = imp.config.borrow();
        let monitors = load_monitors();

        let favorites = config.get_favorites();
        if !favorites.is_empty() {
            imp.list_box.append(&create_header_row("⭐ Quick Switch"));

            for fav in &favorites {
                if let Some(monitor) = monitors.iter().find(|m| m.id == fav.monitor_id) {
                    let input = InputSource::from_vcp_value(fav.input_value);
                    let is_current = monitor.current_input == Some(input);
                    let display_name = get_input_display_name(&config, &monitor.id, input);
                    let label = format!("{} → {}", display_name, monitor.name);

                    let row = MonitorSwitchInputRow::new(&label, is_current, monitor.index, input);
                    imp.list_box.append(&row);
                }
            }

            imp.list_box.append(&create_separator_row());
        }

        for monitor in &monitors {
            imp.list_box.append(&create_header_row(&monitor.name));

            for &input in &monitor.available_inputs {
                let is_current = monitor.current_input == Some(input);
                let is_favorite = config.is_favorite(&monitor.id, input.to_vcp_value());
                let display_name = get_input_display_name(&config, &monitor.id, input);
                let label = if is_favorite {
                    format!("⭐ {}", display_name)
                } else {
                    display_name
                };

                let row = MonitorSwitchInputRow::new(&label, is_current, monitor.index, input);
                imp.list_box.append(&row);
            }

            imp.list_box.append(&create_separator_row());
        }
    }

    fn switch_input(&self, monitor_index: usize, input: InputSource) {
        let mut monitors = Monitor::enumerate();
        if let Some(monitor) = monitors.get_mut(monitor_index) {
            let _ = monitor.set_input(input);
        }
        self.refresh();
    }

    pub fn config(&self) -> std::cell::Ref<Config> {
        self.imp().config.borrow()
    }

    pub fn update_config<F>(&self, f: F)
    where
        F: FnOnce(&mut Config),
    {
        f(&mut self.imp().config.borrow_mut());
    }
}

fn load_monitors() -> Vec<MonitorData> {
    let mut monitors = Monitor::enumerate();
    monitors
        .iter_mut()
        .enumerate()
        .map(|(index, monitor)| {
            let id = monitor.id();
            let name = monitor
                .model_name()
                .or_else(|| monitor.manufacturer_id())
                .unwrap_or_else(|| format!("Monitor {}", index + 1));
            let current_input = monitor.get_current_input().ok();
            let available_inputs = monitor.get_available_inputs().unwrap_or_default();

            MonitorData {
                id,
                name,
                index,
                current_input,
                available_inputs,
            }
        })
        .collect()
}

fn get_input_display_name(config: &Config, monitor_id: &str, input: InputSource) -> String {
    config
        .get_alias(monitor_id, input.to_vcp_value())
        .map(|s| s.to_string())
        .unwrap_or_else(|| input.name().to_string())
}

fn create_header_row(text: &str) -> ListBoxRow {
    let label = Label::builder()
        .label(text)
        .halign(Align::Start)
        .css_classes(["heading"])
        .build();

    ListBoxRow::builder()
        .activatable(false)
        .selectable(false)
        .child(&label)
        .build()
}

fn create_separator_row() -> ListBoxRow {
    ListBoxRow::builder()
        .activatable(false)
        .selectable(false)
        .child(&Separator::new(Orientation::Horizontal))
        .build()
}

