use clap::ValueEnum as _;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::jvm::jvm::JVM;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::component::list::List;
use crate::tui::page::jvm::install_option::InstallOption;

pub struct JVMOption {
	list: List<'static>,
}

impl Default for JVMOption {
	fn default() -> Self {
		Self {
			list: List::new(JVM::value_variants().iter().map(JVM::to_string)),
		}
	}
}

impl InstallOption for JVMOption {}

impl Component for JVMOption {
	type Return = ();

	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		self.list.propagate_events(app)
	}

	fn render(&self, frame: &mut Frame, area: Rect, app: &FujiApp) -> Self::Return {
		self.list.render(frame, area, app);
	}
}
