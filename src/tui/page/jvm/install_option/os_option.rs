#![cfg(feature = "tui")]
#![cfg(feature = "multi-os")]
use clap::ValueEnum as _;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::install_option;
use crate::os::OS;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::component::list::{List, ListEntry};
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
