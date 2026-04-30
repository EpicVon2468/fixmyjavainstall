#![cfg(feature = "tui")]
use clap::ValueEnum as _;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::jvm::jvm::JVM;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::component::list::{List, ListEntry};
use crate::tui::page::jvm::install_option::InstallOption;

impl ListEntry for JVM {
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

pub struct JVMOption {
	list: List<'static>,
}

impl Default for JVMOption {
	fn default() -> Self {
		Self {
			list: List::from(JVM::value_variants()),
		}
	}
}

impl InstallOption for JVMOption {
	fn tab_name(&self) -> &'static str {
		"Build/Vendor"
	}
}

impl Component for JVMOption {
	type Return = ();

	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		self.list.propagate_events(app)
	}

	fn render(&self, frame: &mut Frame, area: Rect, app: &FujiApp) -> Self::Return {
		self.list.render(frame, area, app);
	}
}
