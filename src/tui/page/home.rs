use ratatui::Frame;
use ratatui::layout::Rect;

use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::page::Page;

pub struct HomePage;

impl HomePage {
	pub const fn new() -> Self {
		Self {}
	}
}

impl Page for HomePage {
	fn propagate_page_events(&mut self, app: &FujiApp) -> (bool, Option<Box<dyn Page>>) {
		let consumed: bool = self.propagate_events(app);
		(consumed, None)
	}
}

impl Component for HomePage {
	type Return = ();

	fn propagate_events(&mut self, _app: &FujiApp) -> bool {
		false
	}

	fn render(&mut self, _frame: &mut Frame, _area: Rect, _app: &mut FujiApp) -> Self::Return {}
}
