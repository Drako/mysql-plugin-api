use std::ffi::c_void;
use std::os::raw::c_ulong;

/// For each client connection we create a separate thread with THD serving as a thread/connection descriptor.
type THD = c_void;

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
#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
#[repr(C)]
pub struct StorageEngineInfo {
    /// This needs to be set to `MYSQL_HANDLERTON_INTERFACE_VERSION`.
    pub interface_version: i32,
}

#[derive(Eq, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
#[allow(missing_docs)]
#[repr(C)]
pub enum ShowCompOption {
    Yes,
    No,
    Disabled,
}

/// Legacy Database Type
#[derive(Eq, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
#[allow(missing_docs)]
#[repr(C)]
pub enum LegacyDbType {
    Unknown = 0,
    DiabIsam = 1,
    Hash,
    MIsam,
    PIsam,
    RmsIsam,
    Heam,
    Isam,
    MrgIsam,
    MyIsam,
    MrgMyIsam,
    BerkeleyDb,
    InnoDb,
    Gemini,
    NDbCluster,
    ExampleDb,
    ArchiveDb,
    CsvDb,
    FederatedDb,
    BlackholeDb,
    PartitionDb,
    // No longer used.
    Binlog,
    Solid,
    Pbxt,
    TableFunction,
    MemCache,
    Falcon,
    Maria,
    /** Performance schema engine. */
    PerformanceSchema,
    TempTable,
    FirstDynamic = 42,
    Default = 127,  // Must be last
}

/// Close-connection notification.
pub type CloseConnectionFunc = extern "C" fn(hton: *mut Handlerton, thd: *mut THD) -> i32;

/// Terminate-connection/statement notification.
pub type KillConnectionFunc = extern "C" fn(hton: *mut Handlerton, thd: *mut THD);

/// Shut down all storage engine background tasks that might access the data dictionary, before the main shutdown.
pub type PreDdShutdownFunc = extern "C" fn(hton: *mut Handlerton);

/// The HANDLER-singleTON.
#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
#[allow(missing_docs)]
#[repr(C)]
pub struct Handlerton {
    /// Whether the plugin should be shown.
    pub state: ShowCompOption,

    /// Historical number used for frm file to determine the correct storage engine.
    pub db_type: LegacyDbType,

    /// Memory area of the storage engine.
    pub slot: u32,

    /// To store per-savepoint data storage engine is provided with an area of a requested size (0 is ok here).
    pub savepoint_offset: u32,

    pub close_connection: *const CloseConnectionFunc,

    pub kill_connection: *const KillConnectionFunc,

    pub pre_dd_shutdown: *const PreDdShutdownFunc,
}

/// Function to invoke when plugin is loaded.
pub type InitFunc = extern "C" fn(*mut c_void) -> i32;

/// Function to invoke when plugin is uninstalled.
pub type CheckUninstallFunc = extern "C" fn(*mut c_void) -> i32;

/// Function to invoke when plugin is unloaded.
pub type DeinitFunc = extern "C" fn(*mut c_void) -> i32;

/// A plugin description.
#[cfg_attr(test, derive(Debug))]
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

    /// Function to invoke when plugin is loaded.
    pub init: *const InitFunc,

    /// Function to invoke when plugin is uninstalled.
    pub check_uninstall: *const CheckUninstallFunc,

    /// Function to invoke when plugin is unloaded.
    pub deinit: *const DeinitFunc,

    /// Version number of the plugin.
    pub version: u32,

    /// TODO: SHOW STATUS Server status variable
    pub status_vars: *const c_void,

    /// TODO: Definition of system vars structure for access their information in the plugin
    pub system_vars: *const c_void,

    /// Reserved for dependency checking.
    pub reserved: *const c_void,

    /// Flags for the plugin.
    pub flags: c_ulong,
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
            init: std::ptr::null(),
            check_uninstall: std::ptr::null(),
            deinit: std::ptr::null(),
            version: 0,
            status_vars: std::ptr::null(),
            system_vars: std::ptr::null(),
            reserved: std::ptr::null(),
            flags: 0,
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
            ..Plugin::zero()
        };

        let _plugins: [Plugin; 2] = [
            plugin,
            Plugin::zero(),
        ];
    }
}
