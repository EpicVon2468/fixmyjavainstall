#![cfg(feature = "tui")]
use ratatui::Frame;
use ratatui::crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Layout, Rect};

use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::component::logo::FujiLogo;
use crate::tui::page::Page;
use crate::tui::page::jvm::JVMPage;

pub struct HomePage {
	logo: FujiLogo,
	layout: Layout,
}

impl Default for HomePage {
	fn default() -> Self {
		Self {
			logo: Default::default(),
			layout: Layout::vertical([Constraint::Length(7), Constraint::Fill(1)]),
		}
	}
}

impl Page for HomePage {
	fn propagate_page_events(&mut self, app: &FujiApp) -> (bool, Option<Box<dyn Page>>) {
		if self.propagate_events(app) {
			return (true, None);
		};
		if app.is_key_down(KeyCode::Enter) {
			(true, Some(Box::new(JVMPage::default())))
		} else {
			(false, None)
		}
	}
}

impl Component for HomePage {
	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		self.logo.propagate_events(app)
	}

	fn render(&self, frame: &mut Frame, area: Rect, app: &FujiApp) -> Self::Return {
		let [top, _bottom] = area.layout(&self.layout);
		self.logo.render(frame, top, app);
	}
}
