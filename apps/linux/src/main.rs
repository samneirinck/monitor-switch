use ksni::blocking::TrayMethods;
use ksni::menu::*;
use ksni::{MenuItem, Tray};
use monitor_core::{Config, InputSource, Monitor};
use std::sync::{Arc, Mutex};

struct MonitorData {
    id: String,
    name: String,
    index: usize,
    current_input: Option<InputSource>,
    available_inputs: Vec<InputSource>,
}

struct MonitorTray {
    monitors: Arc<Mutex<Vec<MonitorData>>>,
    config: Arc<Mutex<Config>>,
}

impl MonitorTray {
    fn new() -> Self {
        let tray = MonitorTray {
            monitors: Arc::new(Mutex::new(Vec::new())),
            config: Arc::new(Mutex::new(Config::load())),
        };
        tray.refresh_monitors();
        tray
    }

    fn refresh_monitors(&self) {
        let mut monitors_data = Vec::new();
        let mut monitors = Monitor::enumerate();

        for (index, monitor) in monitors.iter_mut().enumerate() {
            let id = monitor.id();
            let name = monitor
                .model_name()
                .or_else(|| monitor.manufacturer_id())
                .unwrap_or_else(|| format!("Monitor {}", index));
            let current_input = monitor.get_current_input().ok();
            let available_inputs = monitor.get_available_inputs().unwrap_or_default();

            monitors_data.push(MonitorData {
                id,
                name,
                index,
                current_input,
                available_inputs,
            });
        }

        *self.monitors.lock().unwrap() = monitors_data;
    }

    fn get_input_label(&self, monitor_id: &str, input: InputSource, is_current: bool) -> String {
        let config = self.config.lock().unwrap();
        let alias = config.get_alias(monitor_id, input.to_vcp_value());
        let is_favorite = config.is_favorite(monitor_id, input.to_vcp_value());

        let name = alias.map(|s| s.to_string()).unwrap_or_else(|| input.name().to_string());
        let prefix = if is_favorite { "⭐ " } else { "" };
        let suffix = if is_current { " ✓" } else { "" };
        format!("{}{}{}", prefix, name, suffix)
    }
}

impl Tray for MonitorTray {
    fn id(&self) -> String {
        "monitor-switch".into()
    }

    fn icon_name(&self) -> String {
        "video-display".into()
    }

    fn title(&self) -> String {
        "Monitor Switch".into()
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let mut items: Vec<MenuItem<Self>> = Vec::new();
        let monitors = self.monitors.lock().unwrap();
        let config = self.config.lock().unwrap();

        let favorites = config.get_favorites();
        if !favorites.is_empty() {
            items.push(
                StandardItem {
                    label: "⭐ Quick Switch".into(),
                    enabled: false,
                    ..Default::default()
                }
                .into(),
            );

            for fav in favorites {
                let monitor_id = fav.monitor_id.clone();
                let input_value = fav.input_value;
                let input = InputSource::from_vcp_value(input_value);

                if let Some(monitor) = monitors.iter().find(|m| m.id == monitor_id) {
                    let is_current = monitor.current_input == Some(input);
                    let label = format!(
                        "  {} ({})",
                        self.get_input_label(&monitor_id, input, false),
                        monitor.name
                    );
                    let label_with_check = if is_current {
                        format!("{} ✓", label)
                    } else {
                        label
                    };
                    let monitor_index = monitor.index;

                    items.push(
                        StandardItem {
                            label: label_with_check,
                            activate: Box::new(move |tray: &mut Self| {
                                tray.switch_input(monitor_index, input);
                            }),
                            ..Default::default()
                        }
                        .into(),
                    );
                }
            }
            items.push(MenuItem::Separator);
        }
        drop(config);

        for monitor in monitors.iter() {
            items.push(
                StandardItem {
                    label: monitor.name.clone(),
                    enabled: false,
                    ..Default::default()
                }
                .into(),
            );

            for &input in &monitor.available_inputs {
                let is_current = monitor.current_input == Some(input);
                let label = format!("  {}", self.get_input_label(&monitor.id, input, is_current));
                let monitor_index = monitor.index;

                items.push(
                    StandardItem {
                        label,
                        activate: Box::new(move |tray: &mut Self| {
                            tray.switch_input(monitor_index, input);
                        }),
                        ..Default::default()
                    }
                    .into(),
                );
            }
            items.push(MenuItem::Separator);
        }

        items.push(
            StandardItem {
                label: "Refresh".into(),
                activate: Box::new(|tray: &mut Self| {
                    tray.refresh_monitors();
                    *tray.config.lock().unwrap() = Config::load();
                }),
                ..Default::default()
            }
            .into(),
        );
        items.push(MenuItem::Separator);
        items.push(
            StandardItem {
                label: "Quit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        );

        items
    }
}

impl MonitorTray {
    fn switch_input(&mut self, monitor_index: usize, input: InputSource) {
        let mut monitors = Monitor::enumerate();
        if let Some(monitor) = monitors.get_mut(monitor_index) {
            let _ = monitor.set_input(input);
        }
        self.refresh_monitors();
    }
}

fn main() {
    let tray = MonitorTray::new();
    let _handle = tray.spawn().unwrap();
    loop {
        std::thread::park();
    }
}

