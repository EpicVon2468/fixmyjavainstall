use ratatui::Frame;
use ratatui::layout::Rect;

use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::page::Page;

pub struct HomePage;

impl Page for HomePage {}

impl Component for HomePage {
	type Return = ();

	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		false
	}

	fn render(&mut self, frame: &mut Frame, area: Rect, app: &mut FujiApp) -> Self::Return {}
}
