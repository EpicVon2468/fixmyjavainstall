use console::Key;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::page::Page;
use crate::tui::tab::Tab;

pub struct JVMPage {
	pub tab: Tab,
}

impl Page for JVMPage {}

impl Component for JVMPage {
	type Return = ();

	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		if self.tab.propagate_events(app) {
			return true;
		};
		if app.is_key_down(Key::Backspace) {
			// todo android-like backstack?
			return true;
		};
		false
	}

	fn render(&mut self, frame: &mut Frame, area: Rect, app: &mut FujiApp) -> Self::Return {
		self.tab.render(frame, area, app);
	}
}
