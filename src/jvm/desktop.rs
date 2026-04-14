#![cfg(target_os = "linux")]
use std::fs::File;
use std::io::Write as _;
use std::path::Path;

use anyhow::{Context as _, Result};

// https://specifications.freedesktop.org/desktop-entry/latest/
// /usr/share/applications
pub const FREEDESKTOP_ENTRY: &str = include_str!("../../fuji.java.desktop");
pub const FREEDESKTOP_ENTRY_TERMINAL: &str = include_str!("../../fuji.java.terminal.desktop");

pub fn install_desktop_entries() -> Result<()> {
	let base: &Path = Path::new("/usr/share/applications");
	if !base.exists() {
		return Ok(());
	};
	macro_rules! desktop_entry {
		($output:literal, $ident:ident) => {
			File::create(base.join($output))
				.context(concat!(
					"Couldn't create/write '/usr/share/applications/",
					$output,
					"'!"
				))?
				.write_all($crate::jvm::desktop::$ident.as_bytes())
				.context(concat!(
					"Couldn't write to '/usr/share/applications/",
					$output,
					"'!"
				))?;
		};
	}
	desktop_entry!("fuji.java.desktop", FREEDESKTOP_ENTRY);
	desktop_entry!("fuji.java.terminal.desktop", FREEDESKTOP_ENTRY_TERMINAL);
	Ok(())
}
