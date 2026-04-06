#![cfg(target_os = "linux")]
// https://specifications.freedesktop.org/desktop-entry/latest/
// /usr/share/applications
pub const FREEDESKTOP_ENTRY: &str = include_str!("../../fuji.java.desktop");
pub const FREEDESKTOP_ENTRY_TERMINAL: &str = include_str!("../../fuji.java.terminal.desktop");