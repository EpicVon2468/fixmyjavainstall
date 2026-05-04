#![cfg(feature = "tui")]
use std::sync::LazyLock;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Span};

use crate::tui::INVERTED;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;

#[derive_const(Default)]
pub struct HelpSection;

static LAYOUT: LazyLock<Layout> =
	LazyLock::new(|| Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]));
static TOP_ROW_LAYOUT: LazyLock<Layout> = LazyLock::new(|| {
	Layout::horizontal([
		Constraint::Fill(1),
		Constraint::Fill(1),
		Constraint::Fill(1),
	])
});

impl HelpSection {
	fn render_help(frame: &mut Frame, area: Rect) {
		let [top, bottom] = area.layout(&LAYOUT);
		Self::render_help_top_row(frame, top);
		Self::render_help_bottom_row(frame, bottom);
	}

	fn render_help_top_row(frame: &mut Frame, area: Rect) {
		let [quit, back, select] = area.layout(&TOP_ROW_LAYOUT);
		Self::help_entry(frame, quit, ":q", "Quit");
		Self::help_entry(frame, back, "Backspace", "Back");
		Self::help_entry(frame, select, "Enter", "Select");
	}

	// TODO: have page-specific keybinds show up on this row
	fn render_help_bottom_row(_frame: &mut Frame, area: Rect) {
		let [_] = area.layout(&Self::help_bottom_row_layout());
	}

	fn help_bottom_row_layout() -> Layout {
		Layout::horizontal([Constraint::Fill(1)])
	}

	fn help_entry(frame: &mut Frame, area: Rect, key: &str, action: &str) {
		frame.render_widget(
			Line::from_iter([Span::styled(key, INVERTED), format!(" {action}").into()]),
			area,
		);
	}
}

impl Component for HelpSection {
	fn render(&self, frame: &mut Frame, area: Rect, _app: &FujiApp) -> Self::Return {
		Self::render_help(frame, area);
	}
}
