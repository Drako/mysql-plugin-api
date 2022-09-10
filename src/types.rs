use std::ffi::c_void;

/// The type of plugin being declared.
#[derive(Eq, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
#[non_exhaustive]
#[repr(i32)]
pub enum PluginType {
    /// User-Defined Function.
    Udf = 0,

    /// Storage engine.
    Storage = 1,
}

/// The license under which the plugin is being published.
#[derive(Eq, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
#[repr(i32)]
pub enum License {
    /// A proprietary license.
    Proprietary = 0,

    /// The GNU General Public License.
    Gpl = 1,

    /// The BSD license.
    Bsd = 2,
}

/// Information about a storage engine plugin.
#[repr(C)]
pub struct StorageEngineInfo {
    /// This needs to be set to `MYSQL_HANDLERTON_INTERFACE_VERSION`.
    pub interface_version: i32,
}

/// A plugin description,
#[repr(C)]
pub struct Plugin {
    /// The type of the plugin.
    pub plugin_type: PluginType,

    /// Pointer to one of the supported info structures.
    ///
    /// These are:
    ///  * `StorageEngineInfo`
    pub info: *const c_void,

    /// The name of the plugin.
    pub name: *const u8,

    /// The author (Person or organization).
    pub author: *const u8,

    /// The description.
    pub descr: *const u8,

    /// The license under which the plugin is published.
    pub license: License,
}

unsafe impl Sync for Plugin {}

impl Plugin {
    /// Create a Plugin information structure with all fields being zero.
    pub const fn zero() -> Self {
        Self {
            plugin_type: PluginType::Udf,
            info: std::ptr::null_mut(),
            name: std::ptr::null(),
            author: std::ptr::null(),
            descr: std::ptr::null(),
            license: License::Proprietary,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::c_void;
    use crate::constants::MYSQL_HANDLERTON_INTERFACE_VERSION;
    use crate::types::{License, Plugin, PluginType, StorageEngineInfo};

    #[test]
    fn create_plugin_struct() {
        let info = StorageEngineInfo {
            interface_version: MYSQL_HANDLERTON_INTERFACE_VERSION,
        };

        let plugin = Plugin {
            plugin_type: PluginType::Storage,
            info: &info as *const _ as *const c_void,
            name: b"example\0" as *const u8,
            author: b"Felix Bytow\0" as *const u8,
            descr: b"Example storage engine in Rust\0" as *const u8,
            license: License::Bsd,
        };

        let _plugins: [Plugin; 2] = [
            plugin,
            Plugin::zero(),
        ];
    }
}
