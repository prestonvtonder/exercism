// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    #[cfg(feature = "add-a-variant")]
    Debug,
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        #[cfg(feature = "add-a-variant")]
        LogLevel::Debug   => debug(message),
        LogLevel::Error   => error(message),
        LogLevel::Info    => info(message),
        LogLevel::Warning => warn(message),
    }
}
pub fn info(message: &str) -> String {
    format!("[INFO]: {}", message)
}
pub fn warn(message: &str) -> String {
    format!("[WARNING]: {}", message)
}
pub fn error(message: &str) -> String {
    format!("[ERROR]: {}", message)
}
#[cfg(feature = "add-a-variant")]
pub fn debug(message: &str) -> String {
    format!("[DEBUG]: {}", message)
}
