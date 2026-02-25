use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, CheckButton, Image, Label, ListBox, ListBoxRow, Orientation, SelectionMode, Separator};
use libadwaita as adw;
use libadwaita::prelude::*;
use monitor_core::{Config, InputSource, Monitor};
use std::cell::RefCell;
use std::rc::Rc;

use crate::autostart::Autostart;
use crate::preferences::PreferencesWindow;

pub struct MainWindow {
    pub window: adw::ApplicationWindow,
}

struct MonitorData {
    id: String,
    name: String,
    index: usize,
    current_input: Option<InputSource>,
    available_inputs: Vec<InputSource>,
}

impl MainWindow {
    pub fn new(app: &adw::Application) -> Self {
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Monitor Switch")
            .default_width(320)
            .default_height(400)
            .resizable(false)
            .build();

        let config = Rc::new(RefCell::new(Config::load()));
        let content = build_content(&window, &config);
        window.set_content(Some(&content));

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}

fn build_content(window: &adw::ApplicationWindow, config: &Rc<RefCell<Config>>) -> GtkBox {
    let content = GtkBox::new(Orientation::Vertical, 0);
    content.add_css_class("main-content");

    let list_box = ListBox::new();
    list_box.set_selection_mode(SelectionMode::None);
    list_box.add_css_class("boxed-list");
    list_box.set_margin_start(12);
    list_box.set_margin_end(12);
    list_box.set_margin_top(12);
    list_box.set_margin_bottom(12);

    let monitors = load_monitors();
    let config_ref = config.borrow();

    let favorites = config_ref.get_favorites();
    if !favorites.is_empty() {
        let header = create_header_row("⭐ Quick Switch");
        list_box.append(&header);

        for fav in &favorites {
            if let Some(monitor) = monitors.iter().find(|m| m.id == fav.monitor_id) {
                let input = InputSource::from_vcp_value(fav.input_value);
                let is_current = monitor.current_input == Some(input);
                let display_name = get_input_display_name(&config_ref, &monitor.id, input);
                let label = format!("{} → {}", display_name, monitor.name);

                let row = create_input_row(&label, is_current, true, monitor.index, input);
                list_box.append(&row);
            }
        }

        list_box.append(&create_separator_row());
    }

    for monitor in &monitors {
        let header = create_header_row(&monitor.name);
        list_box.append(&header);

        for &input in &monitor.available_inputs {
            let is_current = monitor.current_input == Some(input);
            let is_favorite = config_ref.is_favorite(&monitor.id, input.to_vcp_value());
            let display_name = get_input_display_name(&config_ref, &monitor.id, input);
            let label = if is_favorite {
                format!("⭐ {}", display_name)
            } else {
                display_name
            };

            let row = create_input_row(&label, is_current, false, monitor.index, input);
            list_box.append(&row);
        }

        list_box.append(&create_separator_row());
    }
    drop(config_ref);

    let scrolled = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .child(&list_box)
        .build();

    content.append(&scrolled);
    content.append(&create_separator());
    content.append(&build_footer(window, config));

    content
}

fn build_footer(window: &adw::ApplicationWindow, config: &Rc<RefCell<Config>>) -> GtkBox {
    let footer = GtkBox::new(Orientation::Vertical, 0);
    footer.set_margin_start(12);
    footer.set_margin_end(12);
    footer.set_margin_top(8);
    footer.set_margin_bottom(12);

    let button_box = GtkBox::new(Orientation::Horizontal, 8);

    let prefs_button = Button::builder()
        .label("Preferences")
        .hexpand(true)
        .build();

    let window_clone = window.clone();
    let config_clone = config.clone();
    prefs_button.connect_clicked(move |_| {
        let prefs = PreferencesWindow::new(&window_clone, &config_clone);
        prefs.present();
    });

    let refresh_button = Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_text("Refresh")
        .build();

    let window_clone = window.clone();
    let config_clone = config.clone();
    refresh_button.connect_clicked(move |_| {
        *config_clone.borrow_mut() = Config::load();
        let new_content = build_content(&window_clone, &config_clone);
        window_clone.set_content(Some(&new_content));
    });

    button_box.append(&prefs_button);
    button_box.append(&refresh_button);
    footer.append(&button_box);

    let autostart_check = CheckButton::builder()
        .label("Launch at Login")
        .active(Autostart::is_enabled())
        .margin_top(8)
        .build();

    autostart_check.connect_toggled(|check| {
        if check.is_active() {
            Autostart::enable();
        } else {
            Autostart::disable();
        }
    });

    footer.append(&autostart_check);

    footer
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

fn create_separator() -> Separator {
    Separator::new(Orientation::Horizontal)
}

fn create_input_row(label: &str, is_current: bool, _is_quick_switch: bool, monitor_index: usize, input: InputSource) -> ListBoxRow {
    let hbox = GtkBox::new(Orientation::Horizontal, 8);
    hbox.set_margin_start(8);
    hbox.set_margin_end(8);
    hbox.set_margin_top(8);
    hbox.set_margin_bottom(8);

    let label_widget = Label::builder()
        .label(label)
        .halign(Align::Start)
        .hexpand(true)
        .build();

    hbox.append(&label_widget);

    if is_current {
        let check = Image::from_icon_name("object-select-symbolic");
        check.add_css_class("success");
        hbox.append(&check);
    }

    let row = ListBoxRow::builder()
        .activatable(true)
        .selectable(false)
        .child(&hbox)
        .build();

    row.connect_activated(move |_| {
        switch_input(monitor_index, input);
    });

    row
}

fn switch_input(monitor_index: usize, input: InputSource) {
    let mut monitors = Monitor::enumerate();
    if let Some(monitor) = monitors.get_mut(monitor_index) {
        let _ = monitor.set_input(input);
    }
}

