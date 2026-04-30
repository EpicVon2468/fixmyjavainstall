#![cfg(feature = "tui")]
use clap::ValueEnum as _;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::os::OS;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::component::list::{List, ListEntry};
use crate::tui::page::jvm::install_option::InstallOption;

impl ListEntry for OS {
	fn long_name(&self) -> &'static str {
		match *self {
			// I'd like to interject—  NO!  YOU CAN COMPILE THE KERNEL WITH CLANG AND NOT GCC!  THERE ARE RUST VERSIONS OF THE COREUTILS!
			Self::Linux => "Linux",
			Self::OSX => "macOS",
			Self::Windows => "Windows",
		}
	}
}

pub struct OSOption {
	list: List<'static>,
}

impl Default for OSOption {
	fn default() -> Self {
		Self {
			list: List::from(OS::value_variants()),
		}
	}
}

impl InstallOption for OSOption {
	fn tab_name(&self) -> &'static str {
		"Operating System"
	}
}

impl Component for OSOption {
	type Return = ();

	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		self.list.propagate_events(app)
	}

	fn render(&self, frame: &mut Frame, area: Rect, app: &FujiApp) -> Self::Return {
		self.list.render(frame, area, app);
	}
}
