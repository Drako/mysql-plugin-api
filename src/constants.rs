/// The MySQL version that was used as the base for this library.
///
/// TODO: Make MySQL version configurable
pub const MYSQL_VERSION_ID: i32 = 80030;

/// The MySQL plugin API version.
pub const MYSQL_PLUGIN_INTERFACE_VERSION: i32 = 0x010B;

/// The Handlerton versions of different releases are incompatible.
pub const MYSQL_HANDLERTON_INTERFACE_VERSION: i32 = MYSQL_VERSION_ID << 8;
