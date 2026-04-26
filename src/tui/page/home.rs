use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Text;

use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::page::Page;

pub struct HomePage;

impl HomePage {
	pub const fn new() -> Self {
		Self {}
	}

	fn layout() -> Layout {
		Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).spacing(1)
	}

	pub const LOGO: &'static str = include_str!("../../../doc/logo.txt");
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

	fn render(&mut self, frame: &mut Frame, area: Rect, _app: &mut FujiApp) -> Self::Return {
		let [top, _bottom] = area.layout(&Self::layout());
		frame.render_widget(Text::raw(Self::LOGO).centered(), top);
	}
}
