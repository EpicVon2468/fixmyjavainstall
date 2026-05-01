#![cfg(feature = "tui")]
use clap::ValueEnum as _;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::arch::Arch;
use crate::install_option;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::component::list::{List, ListEntry};
use crate::tui::page::jvm::install_option::InstallOption;

impl const ListEntry for Arch {
	fn name(&self) -> &'static str {
		match *self {
			Self::X64 => "x86-64",
			Self::Aarch64 => "AArch64",
			Self::Riscv64 => "RISC-V64",
		}
	}
}

pub struct ArchOption<'a> {
	list: List<'a>,
}

install_option!(ArchOption, Arch);

impl const InstallOption for ArchOption<'_> {
	fn tab_name(&self) -> &'static str {
		"Architecture"
	}
}
