#![cfg(feature = "tui")]
pub mod install_option;

use ratatui::Frame;
use ratatui::crossterm::event::KeyCode;
use ratatui::layout::{Offset, Rect};
use ratatui::widgets::Tabs;

use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::page::Page;
use crate::tui::page::home::HomePage;
use crate::tui::page::jvm::install_option::InstallOption;
use crate::tui::page::jvm::install_option::arch_option::ArchOption;
use crate::tui::page::jvm::install_option::feature_option::FeatureOption;
use crate::tui::page::jvm::install_option::jvm_option::JVMOption;
use crate::tui::page::jvm::install_option::method_option::MethodOption;
#[cfg(feature = "multi-os")]
use crate::tui::page::jvm::install_option::os_option::OSOption;

pub struct JVMPage {
	selected: usize,
	tabs: Vec<Box<dyn InstallOption>>,
}

impl Default for JVMPage {
	fn default() -> Self {
		Self {
			selected: 0,
			tabs: vec![
				Box::new(JVMOption::default()),
				Box::new(ArchOption::default()),
				#[cfg(feature = "multi-os")]
				Box::new(OSOption::default()),
				Box::new(MethodOption::default()),
				Box::new(FeatureOption::default()),
			],
		}
	}
}

impl JVMPage {
	fn selected(&self) -> &Box<dyn InstallOption> {
		self.tabs.get(self.selected).unwrap()
	}

	fn selected_mut(&mut self) -> &mut Box<dyn InstallOption> {
		self.tabs.get_mut(self.selected).unwrap()
	}

	// TODO: can this be cached?
	pub fn tab_names(&self) -> Vec<&str> {
		self.tabs
			.iter()
			.map(|tab: &Box<dyn InstallOption>| tab.tab_name())
			.collect()
	}

	fn shift_left(&mut self) -> &mut Self {
		let mut new: isize = self.selected.cast_signed().saturating_sub(1);
		if new < 0 {
			new = self.tabs.len().saturating_sub(1).cast_signed();
		};
		self.selected = new.cast_unsigned();
		self
	}

	fn shift_right(&mut self) -> &mut Self {
		let mut new: usize = self.selected.saturating_add(1);
		if new >= self.tabs.len() {
			new = 0;
		};
		self.selected = new;
		self
	}
}

impl Page for JVMPage {
	fn title(&self) -> Option<&'static str> {
		"Installation Configuration".into()
	}

	fn propagate_page_events(&mut self, app: &FujiApp) -> (bool, Option<Box<dyn Page>>) {
		if self.propagate_events(app) {
			return (true, None);
		};
		if app.is_key_down(KeyCode::Backspace) {
			(true, Some(Box::new(HomePage::default())))
		} else {
			(false, None)
		}
	}
}

impl Component for JVMPage {
	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		if self.selected_mut().propagate_events(app) {
			return true;
		};
		if app.is_key_down(KeyCode::Left) {
			self.shift_left();
			return true;
		};
		if app.is_key_down(KeyCode::Right) {
			self.shift_right();
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
