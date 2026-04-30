#![cfg(feature = "tui")]
use clap::ValueEnum as _;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::arch::Arch;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::component::list::{List, ListEntry};
use crate::tui::page::jvm::install_option::InstallOption;

impl ListEntry for Arch {
	fn name(&self) -> &'static str {
		match *self {
			Self::X64 => "x86-64",
			Self::Aarch64 => "AArch64",
			Self::Riscv64 => "RISC-V64",
		}
	}
}

pub struct ArchOption {
	list: List<'static>,
}

impl Default for ArchOption {
	fn default() -> Self {
		Self {
			list: List::from(Arch::value_variants(), false),
		}
	}
}

impl InstallOption for ArchOption {
	fn tab_name(&self) -> &'static str {
		"Architecture"
	}
}

impl Component for ArchOption {
	type Return = ();

	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		self.list.propagate_events(app)
	}

	fn render(&self, frame: &mut Frame, area: Rect, app: &FujiApp) -> Self::Return {
		self.list.render(frame, area, app);
	}
}
