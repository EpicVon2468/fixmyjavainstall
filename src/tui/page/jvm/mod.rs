#![cfg(feature = "tui")]
pub mod install_option;

use console::Key;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::page::Page;
use crate::tui::page::home::HomePage;
use crate::tui::page::jvm::install_option::InstallOption;
use crate::tui::page::jvm::install_option::jvm_option::JVMOption;

pub struct JVMPage {
	selected_tab: usize,
	option_tabs: Vec<Box<dyn InstallOption>>,
}

impl JVMPage {
	#[allow(unused)]
	pub fn new() -> Self {
		Self {
			selected_tab: 0,
			option_tabs: vec![Box::new(JVMOption::default())],
		}
	}

	#[allow(clippy::borrowed_box)]
	fn selected_tab(&self) -> &Box<dyn InstallOption> {
		self.option_tabs.get(self.selected_tab).unwrap()
	}

	fn selected_tab_mut(&mut self) -> &mut Box<dyn InstallOption> {
		self.option_tabs.get_mut(self.selected_tab).unwrap()
	}
}

impl Page for JVMPage {
	fn propagate_page_events(&mut self, app: &FujiApp) -> (bool, Option<Box<dyn Page>>) {
		let consumed: bool = self.propagate_events(app);
		if consumed {
			return (true, None);
		};
		if app.is_key_down(Key::Backspace) {
			// todo android-like backstack?
			(true, Some(Box::new(HomePage::default())))
		} else {
			(false, None)
		}
	}
}

impl Component for JVMPage {
	type Return = ();

	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		self.selected_tab_mut().propagate_events(app)
	}

	fn render(&self, frame: &mut Frame, area: Rect, app: &FujiApp) -> Self::Return {
		self.selected_tab().render(frame, area, app);
	}
}
