use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;
use monitor_core::{InputSource, Monitor};
use std::cell::RefCell;
use std::rc::Rc;

use crate::window::MonitorSwitchWindow;

#[derive(Clone)]
struct MonitorData {
    id: String,
    name: String,
    available_inputs: Vec<InputSource>,
    current_input: Option<InputSource>,
}

struct InputRowWidgets {
    input: InputSource,
    row: adw::EntryRow,
    favorite_switch: gtk4::Switch,
}

pub struct PreferencesWindow {
    window: adw::PreferencesWindow,
    main_window: MonitorSwitchWindow,
    monitors: Vec<MonitorData>,
    input_rows: Rc<RefCell<Vec<InputRowWidgets>>>,
    current_monitor_idx: Rc<RefCell<usize>>,
}

impl PreferencesWindow {
    pub fn new(parent: &MonitorSwitchWindow) -> Self {
        let window = adw::PreferencesWindow::builder()
            .title("Preferences")
            .default_width(500)
            .default_height(450)
            .modal(true)
            .transient_for(parent)
            .build();

        let monitors = load_monitors();
        let input_rows = Rc::new(RefCell::new(Vec::new()));
        let current_monitor_idx = Rc::new(RefCell::new(0));

        let prefs = Self {
            window,
            main_window: parent.clone(),
            monitors,
            input_rows,
            current_monitor_idx,
        };

        prefs.build_ui();
        prefs
    }

    fn build_ui(&self) {
        let page = adw::PreferencesPage::new();
        page.set_icon_name(Some("preferences-system-symbolic"));
        page.set_title("Inputs");

        let monitor_group = adw::PreferencesGroup::new();
        monitor_group.set_title("Monitor");

        let monitor_names: Vec<&str> = self.monitors.iter().map(|m| m.name.as_str()).collect();
        let monitor_list = gtk4::StringList::new(&monitor_names);

        let monitor_row = adw::ComboRow::builder()
            .title("Select Monitor")
            .model(&monitor_list)
            .build();

        monitor_group.add(&monitor_row);
        page.add(&monitor_group);

        let inputs_group = adw::PreferencesGroup::new();
        inputs_group.set_title("Inputs");
        inputs_group.set_description(Some("Set aliases and mark favorites"));
        page.add(&inputs_group);

        if !self.monitors.is_empty() {
            self.populate_inputs(&inputs_group, 0);
        }

        let inputs_group_rc = Rc::new(inputs_group);
        let prefs_clone = PreferencesWindowRef {
            main_window: self.main_window.clone(),
            monitors: self.monitors.clone(),
            input_rows: self.input_rows.clone(),
            current_monitor_idx: self.current_monitor_idx.clone(),
        };
        let inputs_group_for_callback = inputs_group_rc.clone();

        monitor_row.connect_selected_notify(move |row| {
            let idx = row.selected() as usize;
            if idx < prefs_clone.monitors.len() {
                prefs_clone.save_current_monitor();
                *prefs_clone.current_monitor_idx.borrow_mut() = idx;
                prefs_clone.populate_inputs(&inputs_group_for_callback, idx);
            }
        });

        self.window.add(&page);

        let prefs_clone = PreferencesWindowRef {
            main_window: self.main_window.clone(),
            monitors: self.monitors.clone(),
            input_rows: self.input_rows.clone(),
            current_monitor_idx: self.current_monitor_idx.clone(),
        };

        self.window.connect_close_request(move |_| {
            prefs_clone.save_current_monitor();
            prefs_clone.main_window.refresh();
            glib::Propagation::Proceed
        });
    }

    fn populate_inputs(&self, group: &adw::PreferencesGroup, monitor_idx: usize) {
        let ref_data = PreferencesWindowRef {
            main_window: self.main_window.clone(),
            monitors: self.monitors.clone(),
            input_rows: self.input_rows.clone(),
            current_monitor_idx: self.current_monitor_idx.clone(),
        };
        ref_data.populate_inputs(group, monitor_idx);
    }

    pub fn present(&self) {
        self.window.present();
    }
}

#[derive(Clone)]
struct PreferencesWindowRef {
    main_window: MonitorSwitchWindow,
    monitors: Vec<MonitorData>,
    input_rows: Rc<RefCell<Vec<InputRowWidgets>>>,
    current_monitor_idx: Rc<RefCell<usize>>,
}

impl PreferencesWindowRef {
    fn populate_inputs(&self, group: &adw::PreferencesGroup, monitor_idx: usize) {
        for row_widgets in self.input_rows.borrow().iter() {
            group.remove(&row_widgets.row);
        }
        self.input_rows.borrow_mut().clear();

        let Some(monitor) = self.monitors.get(monitor_idx) else {
            return;
        };

        let config = self.main_window.config();

        for &input in &monitor.available_inputs {
            let alias = config
                .get_alias(&monitor.id, input.to_vcp_value())
                .map(|s| s.to_string())
                .unwrap_or_default();
            let is_favorite = config.is_favorite(&monitor.id, input.to_vcp_value());
            let is_current = monitor.current_input == Some(input);

            let title = if is_current {
                format!("✓ {}", input.name())
            } else {
                input.name().to_string()
            };

            let row = adw::EntryRow::builder()
                .title(&title)
                .text(&alias)
                .show_apply_button(false)
                .build();

            let favorite_switch = gtk4::Switch::builder()
                .active(is_favorite)
                .valign(gtk4::Align::Center)
                .tooltip_text("⭐ Favorite")
                .build();

            row.add_suffix(&favorite_switch);

            group.add(&row);

            self.input_rows.borrow_mut().push(InputRowWidgets {
                input,
                row,
                favorite_switch,
            });
        }
    }

    fn save_current_monitor(&self) {
        let idx = *self.current_monitor_idx.borrow();
        let Some(monitor) = self.monitors.get(idx) else {
            return;
        };

        self.main_window.update_config(|config| {
            for row_widgets in self.input_rows.borrow().iter() {
                let alias = row_widgets.row.text();
                let is_favorite = row_widgets.favorite_switch.is_active();
                let input_value = row_widgets.input.to_vcp_value();

                if alias.is_empty() {
                    config.remove_alias(&monitor.id, input_value);
                } else {
                    config.set_alias(&monitor.id, input_value, alias.to_string());
                }

                if is_favorite {
                    config.add_favorite(&monitor.id, input_value);
                } else {
                    config.remove_favorite(&monitor.id, input_value);
                }
            }
            config.save();
        });
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
            let available_inputs = monitor.get_available_inputs().unwrap_or_default();
            let current_input = monitor.get_current_input().ok();

            MonitorData { id, name, available_inputs, current_input }
        })
        .collect()
}

use gtk4::glib;

