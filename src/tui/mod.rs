#![cfg(feature = "tui")]

use anyhow::{Context as _, Result};

use console::{Key, Term};

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize as _;
use ratatui::text::Line;
use ratatui::{DefaultTerminal, Frame, try_init, try_restore};

pub fn main() -> Result<()> {
	let terminal: DefaultTerminal = try_init().context("Couldn't initialise ratatui!")?;
	#[expect(clippy::used_underscore_items)]
	_main(terminal)?;
	try_restore().context("Couldn't restore terminal to original state!")?;
	Ok(())
}

fn _main(mut terminal: DefaultTerminal) -> Result<()> {
	let term: Term = Term::stdout();
	loop {
		terminal.draw(render)?;
		// term.read_key() blocks until input is received.
		// Normally I'd complain, but this is probably good to avoid re-computing logic every frame when not needed
		if term.read_key()? == Key::Char('q') {
			break Ok(());
		};
	}
}

fn render(frame: &mut Frame) {
	let layout: Layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
	let [title, main] = frame.area().layout(&layout);
	render_title(&mut *frame, title);
}

fn render_title(frame: &mut Frame, layout: Rect) {
	let title: Line = Line::from("Fix Ur Java Install – A JVM & Kotlin Management Utility.")
		.centered()
		.bold();
	frame.render_widget(title, layout);
}
