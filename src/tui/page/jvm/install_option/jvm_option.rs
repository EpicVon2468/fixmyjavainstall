#![cfg(feature = "tui")]
use clap::ValueEnum as _;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::install_option;
use crate::jvm::jvm::JVM;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::component::list::{List, ListEntry};
use crate::tui::page::jvm::install_option::InstallOption;

impl const ListEntry for JVM {
	fn name(&self) -> &'static str {
		match *self {
			Self::Auto => "Automatic",
			Self::JBR => "JetBrains Runtime",
			Self::JavaSE => "Java Platform, Standard Edition",
			Self::Temurin => "Eclipse Temurin",
			Self::Liberica => "Liberica JDK",
		}
	}
}

pub struct JVMOption<'a> {
	list: List<'a>,
}

install_option!(JVMOption, JVM);

impl const InstallOption for JVMOption<'_> {
	fn tab_name(&self) -> &'static str {
		"Build/Vendor"
	}
}
