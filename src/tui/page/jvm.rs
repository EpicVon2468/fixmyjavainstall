use ratatui::Frame;
use ratatui::layout::Rect;

use crate::tui::app::FujiApp;
use crate::tui::page::Page;
use crate::tui::tab::Tab;

pub struct JVMPage {
	pub tab: Tab,
}

impl Page for JVMPage {
	fn render(&mut self, frame: &mut Frame, area: Rect, app: &mut FujiApp) {
		frame.render_stateful_widget(&mut self.tab, area, app);
	}
}
