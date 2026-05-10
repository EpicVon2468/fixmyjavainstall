#![cfg(feature = "tui")]
use mtc::Component;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize as _;
use ratatui::text::Text;

use unicode_width::UnicodeWidthStr as _;

use crate::tui::app::FujiApp;
use crate::{LONG_VERSION, centred_horizontally, static_anything, static_layout};

// requires at least a vertical Constraint::Length(6) (preferably 7 for the one line of space)
#[derive_const(Default)]
pub struct FujiLogo;

static_layout!(Layout::vertical([
	Constraint::Fill(1),
	Constraint::Length(1),
]));

static_anything!(WIDTH, usize, LONG_VERSION.width());

impl FujiLogo {
	pub const LOGO: &'static str = "\
		██████  ██   ██       ██  ███████\n\
		██      ██   ██       ██    ▐█▌  \n\
		██████  ██   ██       ██    ▐█▌  \n\
		██      ██   ██  ██   ██    ▐█▌  \n\
		██      ███████  ███████  ███████\
	";
}

impl Component<FujiApp> for FujiLogo {
	fn render(&self, frame: &mut Frame, area: Rect, _app: &FujiApp) {
		let [top, bottom] = area.layout(&LAYOUT);
		frame.render_widget(Text::from(Self::LOGO).centered().bold().light_blue(), top);
		frame.render_widget(LONG_VERSION, centred_horizontally!(bottom, *WIDTH));
	}
}
