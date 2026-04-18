use clap::ValueEnum;

use crate::value_enum_extensions;

// TODO: use environment variables to set args for commands (i.e FUJI_UPDATE_ALTERNATIVES_ARGS)
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

impl InstallMethod {
	#[cfg(target_os = "linux")]
	pub const SYSTEM: Self = Self::Symlink;
	#[cfg(target_os = "macos")]
	pub const SYSTEM: Self = Self::Path;
	#[cfg(target_os = "windows")]
	pub const SYSTEM: Self = Self::Path;
	#[cfg(all(
		not(target_os = "linux"),
		not(target_os = "macos"),
		not(target_os = "windows"),
	))]
	pub const SYSTEM: Self = panic!("Unsupported host!");
}

value_enum_extensions!(
	InstallMethod,
	match *self {
		Self::Path => "path",
		Self::Symlink => "symlink",
		Self::UpdateAlternatives => "update-alternatives",
	}
);
