use std::fmt::{Display, Formatter, Result};

use clap::builder::OsStr;
use clap::ValueEnum;

#[derive(ValueEnum, Clone, PartialEq)]
pub enum OS {
	/// Anything running the Linux kernel
	Linux,
	/// macOS by Apple
	#[clap(aliases = vec!["mac", "macos"])]
	OSX,
	/// Windows by Microslop
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