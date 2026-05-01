#![cfg(feature = "tui")]
use clap::ValueEnum as _;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::install_method::InstallMethod;
use crate::install_option;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::component::list::{List, ListEntry};
use crate::tui::page::jvm::install_option::InstallOption;

impl const ListEntry for InstallMethod {
	fn name(&self) -> &str {
		match *self {
			Self::Path => cfg_select! {
				windows => "%PATH%",
				_ => "$PATH",
			},
			Self::Symlink => "Symbolic link",
			Self::UpdateAlternatives => "update-alternatives",
		}
	}
}

pub struct MethodOption<'a> {
	list: List<'a>,
}

install_option!(MethodOption, InstallMethod);

impl const InstallOption for MethodOption<'_> {
	fn tab_name(&self) -> &'static str {
		"Installation Method"
	}
}
