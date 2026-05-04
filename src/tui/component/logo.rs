#![cfg(feature = "tui")]
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::style::Stylize as _;
use ratatui::text::Text;

use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::{LONG_VERSION, static_layout};

// requires at least a vertical Constraint::Length(6) (preferably 7 for the one line of space)
#[derive_const(Default)]
pub struct FujiLogo;

static_layout!(Layout::vertical([
	Constraint::Fill(1),
	Constraint::Length(1)
]));

impl FujiLogo {
	pub const LOGO: &'static str = "\
		██████  ██   ██       ██  ███████\n\
		██      ██   ██       ██    ▐█▌  \n\
		██████  ██   ██       ██    ▐█▌  \n\
		██      ██   ██  ██   ██    ▐█▌  \n\
		██      ███████  ███████  ███████\
	";
}

impl Component for FujiLogo {
	fn render(&self, frame: &mut Frame, area: Rect, _app: &FujiApp) -> Self::Return {
		let [top, bottom] = area.layout(&LAYOUT);
		frame.render_widget(Text::from(Self::LOGO).centered().bold().light_blue(), top);
		frame.render_widget(Line::from(LONG_VERSION).centered(), bottom);
	}
}
