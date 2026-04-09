//! An enumeration of operating systems.
//!
//! The static constant [`OS::SYSTEM`] may provide the host OS on some targets.
//!
//!	The user-facing `multi-os` feature allows this enumeration to be used to configure installations.
use std::fmt::{Display, Formatter, Result};

use clap::ValueEnum;
use clap::builder::OsStr;

/// An enumeration of operating systems
#[derive(ValueEnum, Clone, PartialEq)]
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
	pub const SYSTEM: OS = OS::Linux;
	/// The [`OS`] of the host – macOS.
	#[cfg(target_os = "macos")]
	pub const SYSTEM: OS = OS::OSX;
	/// The [`OS`] of the host – Windows.
	#[cfg(target_os = "windows")]
	pub const SYSTEM: OS = OS::Windows;
	/// The [`OS`] of the host – Unsupported, panic!
	#[cfg(all(
		not(target_os = "linux"),
		not(target_os = "macos"),
		not(target_os = "windows"),
	))]
	pub const SYSTEM: OS = panic!("Unsupported host operating system!");
}

impl From<OS> for OsStr {

	fn from(value: OS) -> Self {
		value.to_string().into()
	}
}

impl Display for OS {

	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"{}",
			match self {
				OS::Linux => "linux",
				OS::OSX => "osx",
				OS::Windows => "windows",
			}
		)
	}
}