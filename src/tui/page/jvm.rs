use console::Key;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::page::Page;
use crate::tui::page::home::HomePage;
use crate::tui::tab::Tab;

pub struct JVMPage {
	pub tab: Tab,
}

impl JVMPage {
	pub const fn new(tab: Tab) -> Self {
		Self { tab }
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
			(true, Some(Box::new(HomePage::new())))
		} else {
			(false, None)
		}
	}
}

impl Component for JVMPage {
	type Return = ();

	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		self.tab.propagate_events(app)
	}

	fn render(&mut self, frame: &mut Frame, area: Rect, app: &mut FujiApp) -> Self::Return {
		self.tab.render(frame, area, app);
	}
}
