#![doc = "An enumeration of operating systems"]
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
	/// Windows by Microslop – <https://www.microsoft.com/en-gb/windows/>
	Windows,
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

#[cfg(target_os = "linux")]
pub const SYSTEM: OS = OS::Linux;
#[cfg(target_os = "macos")]
pub const SYSTEM: OS = OS::OSX;
#[cfg(target_os = "windows")]
pub const SYSTEM: OS = OS::Windows;