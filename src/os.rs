use std::fmt::{Display, Formatter, Result};

use clap::builder::OsStr;
use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum OS {
	/// Anything running the Linux kernel
	Linux,
	/// macOS by Apple
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

pub fn system() -> OS {
	#[cfg(target_os = "linux")]
	return OS::Linux;
	#[cfg(target_os = "macos")]
	return OS::OSX;
	#[cfg(target_os = "windows")]
	return OS::Windows;
}