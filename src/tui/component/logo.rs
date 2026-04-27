#![cfg(feature = "tui")]
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::style::Stylize as _;
use ratatui::text::Text;

use crate::LONG_VERSION;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;

// requires at least a vertical Constraint::Length(6) (preferably 7 for the one line of space)
#[derive(Default)]
pub struct FujiLogo;

impl FujiLogo {
	#[allow(unused)]
	pub const fn new() -> Self {
		Self {}
	}

	pub const LOGO: &'static str = "\
		██████  ██   ██       ██  ███████\n\
		██      ██   ██       ██    ▐█▌  \n\
		██████  ██   ██       ██    ▐█▌  \n\
		██      ██   ██  ██   ██    ▐█▌  \n\
		██      ███████  ███████  ███████\
	";

	fn layout() -> Layout {
		Layout::vertical([Constraint::Fill(1), Constraint::Length(1)])
	}
}

impl Component for FujiLogo {
	type Return = ();

	fn render(&self, frame: &mut Frame, area: Rect, _app: &FujiApp) -> Self::Return {
		let [top, bottom] = area.layout(&Self::layout());
		frame.render_widget(Text::from(Self::LOGO).centered().bold().light_blue(), top);
		frame.render_widget(Line::from(LONG_VERSION).centered(), bottom);
	}
}
