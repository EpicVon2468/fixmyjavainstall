#![cfg(feature = "tui")]
mod state;

use std::time::Duration;

use anyhow::{Context as _, Result};

use ratatui::crossterm::event::{Event, KeyCode, poll, read};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize as _;
use ratatui::text::Line;
use ratatui::{DefaultTerminal, Frame, try_init, try_restore};

use crate::tui::state::FujiState;

pub fn main() -> Result<()> {
	let terminal: DefaultTerminal = try_init().context("Couldn't initialise ratatui!")?;
	#[expect(clippy::used_underscore_items)]
	let result: Result<()> = _main(terminal);
	try_restore().context("Couldn't restore terminal to original state!")?;
	result
}

fn _main(mut terminal: DefaultTerminal) -> Result<()> {
	let mut state: FujiState = FujiState::new();
	loop {
		terminal.draw(|frame: &mut Frame| render(frame, &mut state))?;
		if !poll(Duration::from_millis(0))? {
			if state.event.is_some() {
				state.event.take();
			};
			continue;
		};
		let event: Event = read()?;
		if let Event::Key(key_event) = event
			&& key_event.code == KeyCode::Char('q')
		{
			break Ok(());
		};
		state.event.replace(event);
	}
}

// https://github.com/ratatui/ratatui/blob/main/examples/concepts/state/src/bin/stateful-widget.rs
// https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples
// https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/block.rs
// https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/tabs.rs
fn render(frame: &mut Frame, state: &mut FujiState) {
	let layout: Layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
	let [title, body] = frame.area().layout(&layout);
	render_title(frame, title);
	frame.render_stateful_widget(state.tab, body, state);
}

fn render_title(frame: &mut Frame, area: Rect) {
	let title: Line = Line::from("Fix Ur Java Install – A JVM & Kotlin Management Utility.")
		.centered()
		.bold();
	frame.render_widget(title, area);
}
