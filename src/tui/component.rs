use ratatui::Frame;
use ratatui::layout::Rect;

use crate::tui::app::FujiApp;

pub trait Component {
	fn render(&mut self, frame: &mut Frame, area: Rect, app: &mut FujiApp);
}
