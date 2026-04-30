#![cfg(feature = "tui")]
pub mod install_option;

use console::Key;

use ratatui::Frame;
use ratatui::layout::{Offset, Rect};
use ratatui::widgets::Tabs;

use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::page::Page;
use crate::tui::page::home::HomePage;
use crate::tui::page::jvm::install_option::InstallOption;
use crate::tui::page::jvm::install_option::arch_option::ArchOption;
use crate::tui::page::jvm::install_option::jvm_option::JVMOption;
use crate::tui::page::jvm::install_option::os_option::OSOption;

pub struct JVMPage {
	selected: usize,
	tabs: Vec<Box<dyn InstallOption>>,
}

impl JVMPage {
	#[allow(unused)]
	pub fn new() -> Self {
		Self {
			selected: 0,
			tabs: vec![
				Box::new(JVMOption::default()),
				Box::new(ArchOption::default()),
				Box::new(OSOption::default()),
			],
		}
	}

	#[allow(clippy::borrowed_box)]
	fn selected(&self) -> &Box<dyn InstallOption> {
		self.tabs.get(self.selected).unwrap()
	}

	fn selected_mut(&mut self) -> &mut Box<dyn InstallOption> {
		self.tabs.get_mut(self.selected).unwrap()
	}

	// TODO: can this be cached?
	#[allow(clippy::borrowed_box)]
	pub fn tab_names(&self) -> Vec<&'static str> {
		self.tabs
			.iter()
			.map(|tab: &Box<dyn InstallOption>| tab.tab_name())
			.collect()
	}

	fn shl(&mut self) -> &mut Self {
		let mut new: isize = self.selected.cast_signed().saturating_sub(1);
		if new < 0 {
			new = self.tabs.len().saturating_sub(1).cast_signed();
		};
		self.selected = new.cast_unsigned();
		self
	}

	fn shr(&mut self) -> &mut Self {
		let mut new: usize = self.selected.saturating_add(1);
		if new >= self.tabs.len() {
			new = 0;
		};
		self.selected = new;
		self
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
		if self.selected_mut().propagate_events(app) {
			return true;
		};
		if app.is_key_down(Key::ArrowLeft) {
			self.shl();
			return true;
		};
		if app.is_key_down(Key::ArrowRight) {
			self.shr();
			return true;
		};
		false
	}

	fn render(&self, frame: &mut Frame, area: Rect, app: &FujiApp) -> Self::Return {
		self.selected().render(frame, area, app);
		let tabs: Tabs = Tabs::new(self.tab_names()).select(self.selected);
		frame.render_widget(tabs, area - Offset::new(1, 2));
	}
}
