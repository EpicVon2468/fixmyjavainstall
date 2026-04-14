//! An enumeration of operating systems.
//!
//! The static constant [`OS::SYSTEM`] may provide the host OS on some targets.
//!
//!	The user-facing `multi-os` feature allows this enumeration to be used to configure installations.
use clap::ValueEnum;

use crate::fuji_value_enum;

/// An enumeration of operating systems
#[non_exhaustive]
#[derive(ValueEnum, Clone, PartialEq, Eq)]
pub enum OS {
	/// Anything running the Linux kernel – <https://kernel.org/>
	Linux,
	/// macOS by Apple – <https://www.apple.com/uk/os/macos/>
	#[clap(aliases = vec!["mac", "macos"])]
	OSX,
	/// Windows by Microslop – [https://www.microslop.com/en-gb/windows/](https://www.microsoft.com/en-gb/windows/)
	#[clap(
		help = "Windows by Microslop – <https://www.microsoft.com/en-gb/windows/>",
		aliases = vec!["win", "nt", "slop"]
	)]
	Windows,
}

impl OS {
	/// The [`OS`] of the host – Linux.
	#[cfg(target_os = "linux")]
	pub const SYSTEM: Self = Self::Linux;
	/// The [`OS`] of the host – macOS.
	#[cfg(target_os = "macos")]
	pub const SYSTEM: Self = Self::OSX;
	/// The [`OS`] of the host – Windows.
	#[cfg(target_os = "windows")]
	pub const SYSTEM: Self = Self::Windows;
	/// The [`OS`] of the host – Unsupported, panic!
	#[cfg(all(
		not(target_os = "linux"),
		not(target_os = "macos"),
		not(target_os = "windows"),
	))]
	pub const SYSTEM: Self = panic!("Unsupported host!");
}

fuji_value_enum!(
	OS,
	match {
		Self::Linux => "linux",
		Self::OSX => "osx",
		Self::Windows => "windows",
	}
);
