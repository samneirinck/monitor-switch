use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::cell::RefCell;

use crate::{Config, InputSource, Monitor};

thread_local! {
    static MONITORS: RefCell<Vec<Monitor>> = RefCell::new(Vec::new());
    static CONFIG: RefCell<Config> = RefCell::new(Config::load());
}

#[repr(C)]
pub struct MonitorInfo {
    pub id: *mut c_char,
    pub model_name: *mut c_char,
    pub manufacturer_id: *mut c_char,
}

#[repr(C)]
pub struct MonitorList {
    pub monitors: *mut MonitorInfo,
    pub count: usize,
}

#[repr(C)]
pub struct InputSourceList {
    pub inputs: *mut InputSource,
    pub count: usize,
}

#[no_mangle]
pub extern "C" fn monitor_core_init() {
    let _ = env_logger::try_init();
}

#[no_mangle]
pub extern "C" fn monitor_enumerate() -> MonitorList {
    let monitors = Monitor::enumerate();
    let mut infos: Vec<MonitorInfo> = monitors
        .iter()
        .map(|m| MonitorInfo {
            id: CString::new(m.id()).unwrap().into_raw(),
            model_name: m
                .model_name()
                .map(|s| CString::new(s).unwrap().into_raw())
                .unwrap_or(ptr::null_mut()),
            manufacturer_id: m
                .manufacturer_id()
                .map(|s| CString::new(s).unwrap().into_raw())
                .unwrap_or(ptr::null_mut()),
        })
        .collect();

    let list = MonitorList {
        monitors: infos.as_mut_ptr(),
        count: infos.len(),
    };
    std::mem::forget(infos);

    MONITORS.with(|m| {
        *m.borrow_mut() = monitors;
    });
    list
}

#[no_mangle]
pub extern "C" fn monitor_list_free(list: MonitorList) {
    if list.monitors.is_null() {
        return;
    }
    unsafe {
        let infos = Vec::from_raw_parts(list.monitors, list.count, list.count);
        for info in infos {
            if !info.id.is_null() {
                drop(CString::from_raw(info.id));
            }
            if !info.model_name.is_null() {
                drop(CString::from_raw(info.model_name));
            }
            if !info.manufacturer_id.is_null() {
                drop(CString::from_raw(info.manufacturer_id));
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn monitor_get_current_input(index: usize) -> InputSource {
    MONITORS.with(|m| {
        let mut monitors = m.borrow_mut();
        if index >= monitors.len() {
            return InputSource::Unknown;
        }
        monitors[index]
            .get_current_input()
            .unwrap_or(InputSource::Unknown)
    })
}

#[no_mangle]
pub extern "C" fn monitor_set_input(index: usize, input: InputSource) -> bool {
    MONITORS.with(|m| {
        let mut monitors = m.borrow_mut();
        if index >= monitors.len() {
            return false;
        }
        monitors[index].set_input(input).is_ok()
    })
}

#[no_mangle]
pub extern "C" fn monitor_get_available_inputs(index: usize) -> InputSourceList {
    MONITORS.with(|m| {
        let mut monitors = m.borrow_mut();
        if index >= monitors.len() {
            return InputSourceList {
                inputs: ptr::null_mut(),
                count: 0,
            };
        }

        match monitors[index].get_available_inputs() {
            Ok(mut inputs) => {
                let list = InputSourceList {
                    inputs: inputs.as_mut_ptr(),
                    count: inputs.len(),
                };
                std::mem::forget(inputs);
                list
            }
            Err(_) => InputSourceList {
                inputs: ptr::null_mut(),
                count: 0,
            },
        }
    })
}

#[no_mangle]
pub extern "C" fn input_source_list_free(list: InputSourceList) {
    if !list.inputs.is_null() {
        unsafe {
            drop(Vec::from_raw_parts(list.inputs, list.count, list.count));
        }
    }
}

#[no_mangle]
pub extern "C" fn input_source_name(input: InputSource) -> *const c_char {
    let name = input.name();
    name.as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn config_get_alias(monitor_id: *const c_char, input_value: u16) -> *mut c_char {
    if monitor_id.is_null() {
        return ptr::null_mut();
    }

    let monitor_id = unsafe { CStr::from_ptr(monitor_id) };
    let monitor_id = match monitor_id.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    CONFIG.with(|c| {
        c.borrow()
            .get_alias(monitor_id, input_value)
            .map(|s| CString::new(s).unwrap().into_raw())
            .unwrap_or(ptr::null_mut())
    })
}

#[no_mangle]
pub extern "C" fn config_set_alias(
    monitor_id: *const c_char,
    input_value: u16,
    alias: *const c_char,
) -> bool {
    if monitor_id.is_null() || alias.is_null() {
        return false;
    }

    let monitor_id = unsafe { CStr::from_ptr(monitor_id) };
    let alias = unsafe { CStr::from_ptr(alias) };

    let (monitor_id, alias) = match (monitor_id.to_str(), alias.to_str()) {
        (Ok(m), Ok(a)) => (m, a),
        _ => return false,
    };

    CONFIG.with(|c| {
        let mut config = c.borrow_mut();
        config.set_alias(monitor_id, input_value, alias.to_string());
        config.save().is_ok()
    })
}

#[no_mangle]
pub extern "C" fn config_remove_alias(monitor_id: *const c_char, input_value: u16) -> bool {
    if monitor_id.is_null() {
        return false;
    }

    let monitor_id = unsafe { CStr::from_ptr(monitor_id) };
    let monitor_id = match monitor_id.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    CONFIG.with(|c| {
        let mut config = c.borrow_mut();
        config.remove_alias(monitor_id, input_value);
        config.save().is_ok()
    })
}

#[no_mangle]
pub extern "C" fn config_reload() {
    CONFIG.with(|c| {
        *c.borrow_mut() = Config::load();
    });
}

#[no_mangle]
pub extern "C" fn config_is_favorite(monitor_id: *const c_char, input_value: u16) -> bool {
    if monitor_id.is_null() {
        return false;
    }

    let monitor_id = unsafe { CStr::from_ptr(monitor_id) };
    let monitor_id = match monitor_id.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    CONFIG.with(|c| c.borrow().is_favorite(monitor_id, input_value))
}

#[no_mangle]
pub extern "C" fn config_add_favorite(monitor_id: *const c_char, input_value: u16) -> bool {
    if monitor_id.is_null() {
        return false;
    }

    let monitor_id = unsafe { CStr::from_ptr(monitor_id) };
    let monitor_id = match monitor_id.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    CONFIG.with(|c| {
        let mut config = c.borrow_mut();
        config.add_favorite(monitor_id, input_value);
        config.save().is_ok()
    })
}

#[no_mangle]
pub extern "C" fn config_remove_favorite(monitor_id: *const c_char, input_value: u16) -> bool {
    if monitor_id.is_null() {
        return false;
    }

    let monitor_id = unsafe { CStr::from_ptr(monitor_id) };
    let monitor_id = match monitor_id.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    CONFIG.with(|c| {
        let mut config = c.borrow_mut();
        config.remove_favorite(monitor_id, input_value);
        config.save().is_ok()
    })
}

#[repr(C)]
pub struct FavoriteInfo {
    pub monitor_id: *mut c_char,
    pub input_value: u16,
}

#[repr(C)]
pub struct FavoriteList {
    pub favorites: *mut FavoriteInfo,
    pub count: usize,
}

#[no_mangle]
pub extern "C" fn config_get_favorites() -> FavoriteList {
    CONFIG.with(|c| {
        let config = c.borrow();
        let mut infos: Vec<FavoriteInfo> = config
            .get_favorites()
            .iter()
            .map(|f| FavoriteInfo {
                monitor_id: CString::new(f.monitor_id.clone()).unwrap().into_raw(),
                input_value: f.input_value,
            })
            .collect();

        let list = FavoriteList {
            favorites: infos.as_mut_ptr(),
            count: infos.len(),
        };
        std::mem::forget(infos);
        list
    })
}

#[no_mangle]
pub extern "C" fn favorite_list_free(list: FavoriteList) {
    if list.favorites.is_null() {
        return;
    }
    unsafe {
        let infos = Vec::from_raw_parts(list.favorites, list.count, list.count);
        for info in infos {
            if !info.monitor_id.is_null() {
                drop(CString::from_raw(info.monitor_id));
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn string_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}
