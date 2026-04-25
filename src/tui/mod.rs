#![cfg(feature = "tui")]
mod app;
mod tab;

use anyhow::{Context as _, Result};

use ratatui::layout::Rect;
use ratatui::style::Stylize as _;
use ratatui::text::Line;
use ratatui::{DefaultTerminal, Frame, try_init, try_restore};

use crate::tui::app::FujiApp;

pub fn main() -> Result<()> {
	let terminal: DefaultTerminal = try_init().context("Couldn't initialise ratatui!")?;
	let mut app: FujiApp = FujiApp::new();
	let result: Result<()> = app.main(terminal);
	let _ = try_restore();
	result
}

// https://github.com/ratatui/ratatui/blob/main/examples/concepts/state/src/bin/stateful-widget.rs
// https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples
// https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/block.rs
// https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/tabs.rs

fn render_title(frame: &mut Frame, area: Rect) {
	let title: Line = Line::from("Fix Ur Java Install – A JVM & Kotlin Management Utility.")
		.centered()
		.bold();
	frame.render_widget(title, area);
}
