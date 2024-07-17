/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const LOG_DOMAIN: u32 = 0;
impl LogType {
    /// Third-party application logs
    pub const LOG_APP: LogType = LogType(0);
}
#[repr(transparent)]
/** @brief Enumerates log types.

Currently, <b>LOG_APP</b> is available. \n

@since 8*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LogType(pub ::core::ffi::c_uint);
impl LogLevel {
    /// Debug level to be used by {@link OH_LOG_DEBUG}
    pub const LOG_DEBUG: LogLevel = LogLevel(3);
}
impl LogLevel {
    /// Informational level to be used by {@link OH_LOG_INFO}
    pub const LOG_INFO: LogLevel = LogLevel(4);
}
impl LogLevel {
    /// Warning level to be used by {@link OH_LOG_WARN}
    pub const LOG_WARN: LogLevel = LogLevel(5);
}
impl LogLevel {
    /// Error level to be used by {@link OH_LOG_ERROR}
    pub const LOG_ERROR: LogLevel = LogLevel(6);
}
impl LogLevel {
    /// Fatal level to be used by {@link OH_LOG_FATAL}
    pub const LOG_FATAL: LogLevel = LogLevel(7);
}
#[repr(transparent)]
/** @brief Enumerates log levels.

You are advised to select log levels based on their respective usage scenarios:\n
<ul><li><b>DEBUG</b>: used for debugging and disabled from commercial releases</li> \n
<li><b>INFO</b>: used for logging important system running status and steps in key processes</li> \n
<li><b>WARN</b>: used for logging unexpected exceptions that have little impact on user experience and can
automatically recover. Logs at this level are generally output when such exceptions are detected and
captured.</li> \n
<li><b>ERROR</b>: used for logging malfunction that affects user experience and cannot automatically
recover</li>\n
<li><b>FATAL</b>: used for logging major exceptions that have severely affected user experience and should
not occur.</li></ul> \n

@since 8*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LogLevel(pub ::core::ffi::c_uint);
extern "C" {
    /** @brief Outputs logs.

    You can use this function to output logs based on the specified log type, log level, service domain, log tag,
    and variable parameters determined by the format specifier and privacy identifier in the printf format.

    @param type Indicates the log type. The type for third-party applications is defined by {@link LOG_APP}.
    @param level Indicates the log level, which can be <b>LOG_DEBUG</b>, <b>LOG_INFO</b>, <b>LOG_WARN</b>,
    <b>LOG_ERROR</b>, and <b>LOG_FATAL</b>.
    @param domain Indicates the service domain of logs. Its value is a hexadecimal integer ranging from 0x0 to 0xFFFF.
    @param tag Indicates the log tag, which is a string used to identify the class, file, or service behavior.
    @param fmt Indicates the format string, which is an enhancement of a printf format string and supports the privacy
    identifier. Specifically, {public} or {private} is added between the % character and the format specifier
    in each parameter. \n
    @param ... Indicates a list of parameters. The number and type of parameters must map onto the format specifiers
    in the format string.
    @return Returns <b>0</b> or a larger value if the operation is successful; returns a value smaller
    than <b>0</b> otherwise.
    @since 8*/
    pub fn OH_LOG_Print(
        type_: LogType,
        level: LogLevel,
        domain: ::core::ffi::c_uint,
        tag: *const ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    /** @brief Checks whether logs of the specified service domain, log tag, and log level can be output.

    @param domain Indicates the service domain of logs.
    @param tag Indicates the log tag.
    @param level Indicates the log level.
    @return Returns <b>true</b> if the specified logs can be output; returns <b>false</b> otherwise.
    @since 8*/
    pub fn OH_LOG_IsLoggable(
        domain: ::core::ffi::c_uint,
        tag: *const ::core::ffi::c_char,
        level: LogLevel,
    ) -> bool;
}
