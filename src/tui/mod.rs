#![cfg(feature = "tui")]
mod state;

use anyhow::{Context as _, Result};

use console::{Key, Term};

use ratatui::layout::{Alignment, Constraint, Layout, Offset, Rect};
use ratatui::style::Stylize as _;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Tabs};
use ratatui::{DefaultTerminal, Frame, try_init, try_restore};

use crate::tui::state::{State, Tab};

pub fn main() -> Result<()> {
	let terminal: DefaultTerminal = try_init().context("Couldn't initialise ratatui!")?;
	#[expect(clippy::used_underscore_items)]
	_main(terminal)?;
	try_restore().context("Couldn't restore terminal to original state!")?;
	Ok(())
}

fn _main(mut terminal: DefaultTerminal) -> Result<()> {
	let term: Term = Term::stdout();
	let mut state = State { tab: Tab::Foo };
	loop {
		terminal.draw(|frame: &mut Frame| render(frame, &state))?;
		// term.read_key() blocks until input is received.
		// Normally I'd complain, but this is probably good to avoid re-computing logic every frame when not needed
		match term.read_key()? {
			Key::Char('q') => break Ok(()),
			Key::ArrowLeft => {
				state.tab.shift_self_left();
			},
			Key::ArrowRight => {
				state.tab.shift_self_right();
			},
			_ => (),
		};
	}
}

// https://github.com/ratatui/ratatui/blob/main/examples/concepts/state/src/bin/stateful-widget.rs
// https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples
// https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/block.rs
// https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/tabs.rs
fn render(frame: &mut Frame, state: &State) {
	let layout: Layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
	let [title, main] = frame.area().layout(&layout);
	render_title(&mut *frame, title);
	render_tab_content(&mut *frame, main, state);
	render_tab_list(&mut *frame, main + Offset::new(1, 0), state);
}

fn render_title(frame: &mut Frame, area: Rect) {
	let title: Line = Line::from("Fix Ur Java Install – A JVM & Kotlin Management Utility.")
		.centered()
		.bold();
	frame.render_widget(title, area);
}

#[allow(clippy::as_conversions)]
fn render_tab_list(frame: &mut Frame, area: Rect, state: &State) {
	let tabs: Tabs = Tabs::new(Tab::value_names().to_owned())
		.select(state.tab.ordinal() as usize)
		.padding(" ", " ");
	frame.render_widget(tabs, area);
}

#[allow(unreachable_patterns)]
fn render_tab_content(frame: &mut Frame, area: Rect, state: &State) {
	let text: &str = match state.tab {
		Tab::Foo => "Something something foo",
		Tab::Bar => "Something something bar",
		Tab::Baz => "Something something baz",
		_ => unreachable!(),
	};
	let paragraph: Paragraph = Paragraph::new(text)
		.alignment(Alignment::Center)
		.block(Block::bordered());
	frame.render_widget(paragraph, area);
}
