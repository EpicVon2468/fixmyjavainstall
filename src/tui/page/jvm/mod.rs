#![cfg(feature = "tui")]
pub mod install_option;

use ratatui::Frame;
use ratatui::crossterm::event::KeyCode;
use ratatui::layout::{Offset, Rect};
use ratatui::widgets::Tabs;

use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::component::exit_dialogue::ExitDialogue;
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
	exit_dialogue: ExitDialogue,
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
			exit_dialogue: Default::default(),
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
	#[must_use]
	pub fn tab_names(&self) -> Vec<&str> {
		self.tabs
			.iter()
			.map(|tab: &Box<dyn InstallOption>| tab.tab_name())
			.collect()
	}
}

impl JVMPage {
	fn last_index(&self) -> usize {
		self.tabs.len().saturating_sub(1)
	}

	fn shift_left(&mut self) -> &mut Self {
		let old: usize = self.selected;
		self.selected = if old == 0 {
			self.last_index()
		} else {
			old.saturating_sub(1)
		};
		self
	}

	fn shift_right(&mut self) -> &mut Self {
		let old: usize = self.selected;
		self.selected = if old == self.last_index() {
			0
		} else {
			old.saturating_add(1)
		};
		self
	}
}

impl Page for JVMPage {
	fn title(&self) -> &'static str {
		"Installation Configuration"
	}

	fn propagate_page_events(
		&mut self,
		app: &FujiApp,
	) -> anyhow::Result<(bool, Option<Box<dyn Page>>)> {
		if self.propagate_events(app)? {
			if self.exit_dialogue.should_exit() {
				return Ok((true, Some(Box::new(HomePage::default()))));
			};
			return Ok((true, None));
		};
		Ok((false, None))
	}
}

impl Component for JVMPage {
	fn propagate_events(&mut self, app: &FujiApp) -> anyhow::Result<bool> {
		if self.exit_dialogue.propagate_events(app)? {
			return Ok(true);
		};
		if self.selected_mut().propagate_events(app)? {
			return Ok(true);
		};
		if app.is_key_down(KeyCode::Backspace) {
			self.exit_dialogue.show();
			return Ok(true);
		};
		if app.should_shl() {
			self.shift_left();
			return Ok(true);
		};
		if app.should_shr() {
			self.shift_right();
			return Ok(true);
		};
		Ok(false)
	}

	fn render(&self, frame: &mut Frame, area: Rect, app: &FujiApp) {
		self.selected().render(frame, area, app);
		let tabs: Tabs = Tabs::new(self.tab_names()).select(self.selected);
		frame.render_widget(tabs, area - Offset::new(1, 2));
		self.exit_dialogue.render(frame, area, app);
	}
}
