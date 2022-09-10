#[macro_use]
extern crate mysql_plugin_api;

use mysql_plugin_api::constants::MYSQL_HANDLERTON_INTERFACE_VERSION;
use mysql_plugin_api::types::{StorageEngineInfo, PluginType, License, Plugin};
use std::ffi::c_void;

pub const EXAMPLE_STORAGE_ENGINE: StorageEngineInfo = StorageEngineInfo {
    interface_version: MYSQL_HANDLERTON_INTERFACE_VERSION,
};

mysql_declare_plugin![
    Plugin {
        plugin_type: PluginType::Storage,
        info: &EXAMPLE_STORAGE_ENGINE as *const _ as *const c_void,
        name: b"example\0" as *const u8,
        author: b"Felix Bytow\0" as *const u8,
        descr: b"Example storage engine in Rust\0" as *const u8,
        license: License::Bsd,
    },
];
