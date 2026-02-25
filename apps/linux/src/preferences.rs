use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, CheckButton, DropDown, Entry, Label, ListBox, ListBoxRow, Orientation, SelectionMode, StringList};
use libadwaita as adw;
use libadwaita::prelude::*;
use monitor_core::{Config, InputSource, Monitor};
use std::cell::RefCell;
use std::rc::Rc;

pub struct PreferencesWindow {
    window: adw::Window,
}

struct MonitorData {
    id: String,
    name: String,
    index: usize,
    available_inputs: Vec<InputSource>,
    current_input: Option<InputSource>,
}

impl PreferencesWindow {
    pub fn new(parent: &adw::ApplicationWindow, config: &Rc<RefCell<Config>>) -> Self {
        let window = adw::Window::builder()
            .title("Preferences")
            .default_width(500)
            .default_height(400)
            .modal(true)
            .transient_for(parent)
            .build();

        let monitors = load_monitors();
        let content = build_content(&window, &monitors, config);
        window.set_content(Some(&content));

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
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

            MonitorData { id, name, index, available_inputs, current_input }
        })
        .collect()
}

fn build_content(window: &adw::Window, monitors: &[MonitorData], config: &Rc<RefCell<Config>>) -> GtkBox {
    let content = GtkBox::new(Orientation::Vertical, 0);

    let header = adw::HeaderBar::new();
    content.append(&header);

    let main_box = GtkBox::new(Orientation::Vertical, 12);
    main_box.set_margin_start(12);
    main_box.set_margin_end(12);
    main_box.set_margin_top(12);
    main_box.set_margin_bottom(12);

    let monitor_names: Vec<&str> = monitors.iter().map(|m| m.name.as_str()).collect();
    let monitor_list = StringList::new(&monitor_names);

    let monitor_dropdown = DropDown::builder()
        .model(&monitor_list)
        .build();

    let monitor_box = GtkBox::new(Orientation::Horizontal, 8);
    monitor_box.append(&Label::new(Some("Monitor:")));
    monitor_box.append(&monitor_dropdown);
    main_box.append(&monitor_box);

    let list_box = ListBox::new();
    list_box.set_selection_mode(SelectionMode::None);
    list_box.add_css_class("boxed-list");

    let input_rows: Rc<RefCell<Vec<InputRowData>>> = Rc::new(RefCell::new(Vec::new()));

    let scrolled = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .child(&list_box)
        .build();

    main_box.append(&scrolled);

    if !monitors.is_empty() {
        populate_inputs(&list_box, &monitors[0], config, &input_rows);
    }

    let monitors_rc = Rc::new(monitors.to_vec());
    let list_box_clone = list_box.clone();
    let config_clone = config.clone();
    let input_rows_clone = input_rows.clone();
    monitor_dropdown.connect_selected_notify(move |dropdown| {
        let idx = dropdown.selected() as usize;
        if idx < monitors_rc.len() {
            populate_inputs(&list_box_clone, &monitors_rc[idx], &config_clone, &input_rows_clone);
        }
    });

    let button_box = GtkBox::new(Orientation::Horizontal, 8);
    button_box.set_halign(Align::End);

    let save_button = Button::builder()
        .label("Save")
        .css_classes(["suggested-action"])
        .build();

    let window_clone = window.clone();
    let config_clone = config.clone();
    let monitors_clone = Rc::new(monitors.to_vec());
    let dropdown_clone = monitor_dropdown.clone();
    save_button.connect_clicked(move |_| {
        let idx = dropdown_clone.selected() as usize;
        if idx < monitors_clone.len() {
            let monitor = &monitors_clone[idx];
            save_inputs(&monitor.id, &config_clone, &input_rows);
        }
        window_clone.close();
    });

    button_box.append(&save_button);
    main_box.append(&button_box);
    content.append(&main_box);

    content
}

struct InputRowData {
    input: InputSource,
    alias_entry: Entry,
    favorite_check: CheckButton,
}

fn populate_inputs(list_box: &ListBox, monitor: &MonitorData, config: &Rc<RefCell<Config>>, input_rows: &Rc<RefCell<Vec<InputRowData>>>) {
    while let Some(row) = list_box.first_child() {
        list_box.remove(&row);
    }
    input_rows.borrow_mut().clear();

    let config_ref = config.borrow();

    for &input in &monitor.available_inputs {
        let alias = config_ref
            .get_alias(&monitor.id, input.to_vcp_value())
            .map(|s| s.to_string())
            .unwrap_or_default();
        let is_favorite = config_ref.is_favorite(&monitor.id, input.to_vcp_value());
        let is_current = monitor.current_input == Some(input);

        let row = create_input_row(input, &alias, is_favorite, is_current, input_rows);
        list_box.append(&row);
    }
}

fn create_input_row(input: InputSource, alias: &str, is_favorite: bool, is_current: bool, input_rows: &Rc<RefCell<Vec<InputRowData>>>) -> ListBoxRow {
    let hbox = GtkBox::new(Orientation::Horizontal, 8);
    hbox.set_margin_start(8);
    hbox.set_margin_end(8);
    hbox.set_margin_top(8);
    hbox.set_margin_bottom(8);

    let current_indicator = if is_current { "✓ " } else { "  " };
    let input_label = Label::builder()
        .label(&format!("{}{}", current_indicator, input.name()))
        .width_chars(14)
        .halign(Align::Start)
        .build();

    let alias_entry = Entry::builder()
        .placeholder_text(input.name())
        .text(alias)
        .hexpand(true)
        .build();

    let favorite_check = CheckButton::builder()
        .active(is_favorite)
        .tooltip_text("Favorite")
        .build();

    let star_label = Label::new(Some("⭐"));
    let fav_box = GtkBox::new(Orientation::Horizontal, 4);
    fav_box.append(&favorite_check);
    fav_box.append(&star_label);

    hbox.append(&input_label);
    hbox.append(&alias_entry);
    hbox.append(&fav_box);

    input_rows.borrow_mut().push(InputRowData {
        input,
        alias_entry: alias_entry.clone(),
        favorite_check: favorite_check.clone(),
    });

    ListBoxRow::builder()
        .activatable(false)
        .selectable(false)
        .child(&hbox)
        .build()
}

fn save_inputs(monitor_id: &str, config: &Rc<RefCell<Config>>, input_rows: &Rc<RefCell<Vec<InputRowData>>>) {
    let mut config = config.borrow_mut();

    for row_data in input_rows.borrow().iter() {
        let alias = row_data.alias_entry.text();
        let is_favorite = row_data.favorite_check.is_active();
        let input_value = row_data.input.to_vcp_value();

        if alias.is_empty() {
            config.remove_alias(monitor_id, input_value);
        } else {
            config.set_alias(monitor_id, input_value, alias.to_string());
        }

        if is_favorite {
            config.add_favorite(monitor_id, input_value);
        } else {
            config.remove_favorite(monitor_id, input_value);
        }
    }

    config.save();
}

impl Clone for MonitorData {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            index: self.index,
            available_inputs: self.available_inputs.clone(),
            current_input: self.current_input,
        }
    }
}

