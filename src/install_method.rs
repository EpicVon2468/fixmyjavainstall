use clap::ValueEnum;

use crate::fuji_value_enum;

// TODO: use environment variables to set args for commands (i.e FUJI_UPDATE_ALTERNATIVES_ARGS)
#[non_exhaustive]
#[derive(ValueEnum, Clone)]
pub enum InstallMethod {
	/// Add the directory to the PATH environment variable – (export PATH="$JAVA_HOME:$PATH")
	Path,
	#[cfg(any(unix, feature = "multi-os"))]
	/// Symbolically link the executables from the directory into PATH – (ln -sf $JAVA_HOME/bin/filename /usr/bin/filename)
	Symlink,
	#[cfg(any(target_os = "linux", feature = "multi-os"))]
	/// Use update-alternatives – <https://man7.org/linux/man-pages/man1/update-alternatives.1.html>
	UpdateAlternatives,
}

impl InstallMethod {
	#[cfg(target_os = "linux")]
	pub const SYSTEM: Self = Self::Symlink;
	#[cfg(target_os = "macos")]
	pub const SYSTEM: Self = Self::Path;
	#[cfg(windows)]
	pub const SYSTEM: Self = Self::Path;
	#[cfg(all(
		not(target_os = "linux"),
		not(target_os = "macos"),
		not(windows),
	))]
	pub const SYSTEM: Self = panic!("Unsupported host!");
}

fuji_value_enum!(
	InstallMethod,
	match {
		Self::Path => "path",
		Self::Symlink => "symlink",
		Self::UpdateAlternatives => "update-alternatives",
	}
);
