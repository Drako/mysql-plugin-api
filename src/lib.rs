#![warn(missing_docs)]
#![doc = "Helper library for implementing MySQL plugins in Rust"]

/// Global MySQL constants.
pub mod constants;

/// MySQL data structures.
pub mod types;

/// Macro to help declare MySQL plugins.
///
/// **ATTENTION**: Only use this macro once per plugin crate.
///
/// # Example
///
/// ```rust
/// # use mysql_plugin_api::constants::MYSQL_HANDLERTON_INTERFACE_VERSION;
/// # use mysql_plugin_api::mysql_declare_plugin;
/// # use mysql_plugin_api::types::{Plugin, StorageEngineInfo, PluginType, License};
/// # use std::ffi::c_void;
///
/// pub const EXAMPLE_STORAGE_ENGINE: StorageEngineInfo = StorageEngineInfo {
///     interface_version: MYSQL_HANDLERTON_INTERFACE_VERSION,
/// };
///
/// mysql_declare_plugin![
///     Plugin {
///         plugin_type: PluginType::Storage,
///         info: &EXAMPLE_STORAGE_ENGINE as *const _ as *const c_void,
///         name: b"example\0" as *const u8,
///         author: b"Felix Bytow\0" as *const u8,
///         descr: b"Example storage engine in Rust\0" as *const u8,
///         license: License::Bsd,
///     },
/// ];
/// ```
#[macro_export]
macro_rules! mysql_declare_plugin {
    ($( $plugin:expr ),+ $(,)?) => {
        #[no_mangle]
        pub static _mysql_plugin_interface_version_: i32 = $crate::constants::MYSQL_PLUGIN_INTERFACE_VERSION;

        #[no_mangle]
        pub static _mysql_sizeof_struct_st_plugin_: i32 = std::mem::size_of::<$crate::types::Plugin>() as i32;

        #[no_mangle]
        pub static _mysql_plugin_declarations_: [
            $crate::types::Plugin;
            <[$crate::types::Plugin]>::len(&[$( $plugin, )*]) + 1
        ] = [
            $( $plugin, )*
            $crate::types::Plugin::zero()
        ];
    }
}
