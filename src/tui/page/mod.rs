pub mod home;
pub mod jvm;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::tui::app::FujiApp;

pub trait Page {
	fn render(&mut self, frame: &mut Frame, area: Rect, app: &mut FujiApp);
}
