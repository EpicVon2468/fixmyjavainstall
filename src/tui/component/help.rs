#![cfg(feature = "tui")]
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Span};

use crate::tui::INVERTED;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;

pub struct HelpSection {
	layout: Layout,
	top_row_layout: Layout,
}

impl HelpSection {
	fn render_help(&self, frame: &mut Frame, area: Rect) {
		let [top, bottom] = area.layout(&self.layout);
		self.render_help_top_row(frame, top);
		Self::render_help_bottom_row(frame, bottom);
	}

	fn render_help_top_row(&self, frame: &mut Frame, area: Rect) {
		let [quit, back, select] = area.layout(&self.top_row_layout);
		Self::help_entry(frame, quit, ":q", "Quit");
		Self::help_entry(frame, back, "Backspace", "Back");
		Self::help_entry(frame, select, "Enter", "Select");
	}

	// TODO: have page-specific keybinds show up on this row
	fn render_help_bottom_row(_frame: &mut Frame, area: Rect) {
		let [_] = area.layout(&Self::help_bottom_row_layout());
	}

	fn help_bottom_row_layout() -> Layout {
		Layout::horizontal([Constraint::Fill(1)]).spacing(1)
	}

	fn help_entry(frame: &mut Frame, area: Rect, key: &str, action: &str) {
		frame.render_widget(
			Line::from_iter([Span::styled(key, INVERTED), Span::raw(format!(" {action}"))]),
			area,
		);
	}
}

impl Default for HelpSection {
	fn default() -> Self {
		Self {
			layout: Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]),
			top_row_layout: Layout::horizontal([
				Constraint::Fill(1),
				Constraint::Fill(1),
				Constraint::Fill(1),
			]),
		}
	}
}

impl Component for HelpSection {
	type Return = ();

	fn render(&self, frame: &mut Frame, area: Rect, _app: &FujiApp) -> Self::Return {
		self.render_help(frame, area);
	}
}
