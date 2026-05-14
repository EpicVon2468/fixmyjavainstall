#![cfg(feature = "tui")]

use mtc::{Component, centred_horizontally, static_anything, static_layout};

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize as _;
use ratatui::text::Text;

use unicode_width::UnicodeWidthStr as _;

use crate::fuji_version;
use crate::tui::app::FujiApp;

// requires at least a vertical Constraint::Length(6) (preferably 7 for the one line of space)
#[derive_const(Default)]
pub struct FujiLogo;

static_layout!(Layout::vertical([
	Constraint::Fill(1),
	Constraint::Length(1),
]));

static_anything!(WIDTH, usize, FujiLogo::VERSION.width());
static_anything!(
	LOGO_TEXT,
	Text,
	Text::from(FujiLogo::LOGO).centered().bold().light_blue(),
);

impl FujiLogo {
	pub const LOGO: &'static str = "\
		██████  ██   ██       ██  ███████\n\
		██      ██   ██       ██    ▐█▌  \n\
		██████  ██   ██       ██    ▐█▌  \n\
		██      ██   ██  ██   ██    ▐█▌  \n\
		██      ███████  ███████  ███████\
	";

	const VERSION: &str = concat!('v', fuji_version!());
}

impl Component<FujiApp> for FujiLogo {
	fn render(&self, frame: &mut Frame, area: Rect, _app: &FujiApp) {
		let [top, bottom] = area.layout(&LAYOUT);
		frame.render_widget(&*LOGO_TEXT, top);
		frame.render_widget(Self::VERSION, centred_horizontally!(bottom, *WIDTH));
	}
}
