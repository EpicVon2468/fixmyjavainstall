use std::env::var;

use anyhow::Result;

use clap::ValueEnum;

use dialoguer::Editor;

use crate::value_enum_extensions;

// TODO: use environment variables to set args for commands (i.e FUJI_UPDATE_ALTERNATIVES_ARGS) -- https://crates.io/crates/shell-words/
#[non_exhaustive]
#[derive(ValueEnum, Clone)]
pub enum InstallMethod {
	/// PATH environment variable modification.
	Path,
	#[cfg(any(unix, feature = "multi-os"))]
	/// Symbolic linking.
	#[value(hide = cfg!(all(not(unix), not(feature = "multi-os"))))]
	Symlink,
	/// <https://man7.org/linux/man-pages/man1/update-alternatives.1.html>.
	#[value(hide = cfg!(all(not(target_os = "linux"), not(feature = "multi-os"))))]
	UpdateAlternatives,
}

value_enum_extensions!(
	InstallMethod,
	cfg_select! {
		target_os = "linux" => Self::Symlink,
		target_os = "macos" => Self::Path,
		windows => Self::Path,
		_ => panic!("Unsupported host!"),
	},
	match *self {
		Self::Path => "path",
		Self::Symlink => "symlink",
		Self::UpdateAlternatives => "update-alternatives",
	},
);

fn _test() -> Result<()> {
	let _args: Option<String> = match var("FUJI__CMD__ARGS").map(Some) {
		Ok(value) => value,
		Err(_) => Editor::new().edit("Enter args for <command name here>:")?,
	};
	Ok(())
}
