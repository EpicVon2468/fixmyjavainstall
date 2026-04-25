use ratatui::Frame;
use ratatui::layout::Rect;

use crate::tui::app::FujiApp;
use crate::tui::page::Page;

pub struct HomePage;

impl Page for HomePage {
	fn render(&mut self, frame: &mut Frame, area: Rect, app: &mut FujiApp) {}
}
