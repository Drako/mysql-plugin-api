#[macro_use]
extern crate mysql_plugin_api;

use std::ffi::c_void;

use mysql_plugin_api::constants::MYSQL_HANDLERTON_INTERFACE_VERSION;
use mysql_plugin_api::types::{Handlerton, License, Plugin, PluginType, ShowCompOption, StorageEngineInfo};

pub const EXAMPLE_STORAGE_ENGINE: StorageEngineInfo = StorageEngineInfo {
    interface_version: MYSQL_HANDLERTON_INTERFACE_VERSION,
};

extern "C" fn example_init(p: *mut c_void) -> i32 {
    let hton = p as *mut Handlerton;
    hton.state = ShowCompOption::Yes;
    0
}

mysql_declare_plugin![
    Plugin {
        plugin_type: PluginType::Storage,
        info: &EXAMPLE_STORAGE_ENGINE as *const _ as *const c_void,
        name: b"example\0" as *const u8,
        author: b"Felix Bytow\0" as *const u8,
        descr: b"Example storage engine in Rust\0" as *const u8,
        license: License::Bsd,
        init: &example_init,
        check_uninstall: std::ptr::null(),
        deinit: std::ptr::null(),
        version: 0x0001,
        status_vars: std::ptr::null(),
        system_vars: std::ptr::null(),
        reserved: std::ptr::null(),
        flags: 0,
    },
];
