#![cfg(all(feature = "tui", feature = "multi-os"))]
use mtc::{Component, List, ListEntry};

use crate::install_option;
use crate::os::OS;
use crate::tui::page::jvm::install_option::InstallOption;

impl const ListEntry for OS {
	fn name(&self) -> &'static str {
		match *self {
			// I'd like to interject—  NO!  YOU CAN COMPILE THE KERNEL WITH CLANG AND NOT GCC!  THERE ARE RUST VERSIONS OF THE COREUTILS!
			Self::Linux => "Linux",
			Self::OSX => "macOS",
			Self::Windows => "Windows",
		}
	}
}

pub struct OSOption<'a> {
	list: List<'a>,
}

install_option!(OSOption, OS);

impl const InstallOption for OSOption<'_> {
	fn tab_name(&self) -> &'static str {
		"Operating System"
	}
}
