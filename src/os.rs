//! An enumeration of operating systems.
//!
//! The trait [`Default`] may provide the host OS on some targets.
//!
//!	The user-facing `multi-os` feature allows this enumeration to be used to configure installations.
use clap::ValueEnum;

use crate::value_enum_extensions;

/// An enumeration of operating systems.
#[non_exhaustive]
#[derive(ValueEnum, Clone, PartialEq, Eq)]
pub enum OS {
	/// Anything running the Linux kernel – <https://kernel.org/>.
	Linux,
	/// macOS by Apple – <https://www.apple.com/uk/os/macos/>.
	#[value(aliases = vec!["mac", "macos"])]
	OSX,
	/// Windows by Microslop – [https://www.microslop.com/en-gb/windows/](https://www.microsoft.com/en-gb/windows/).
	#[value(
		help = "Windows by Microslop – https://www.microsoft.com/en-gb/windows/",
		aliases = vec!["win", "nt", "slop"]
	)]
	Windows,
}

value_enum_extensions!(
	OS,
	cfg_select! {
		target_os = "linux" => Self::Linux,
		target_os = "macos" => Self::OSX,
		target_os = "windows" => Self::Windows,
		_ => panic!("Unsupported host!"),
	},
	match *self {
		Self::Linux => "linux",
		Self::OSX => "osx",
		Self::Windows => "windows",
	},
);
