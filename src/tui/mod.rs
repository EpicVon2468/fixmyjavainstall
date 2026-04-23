#![cfg(feature = "tui")]

use std::time::Instant;

use anyhow::{Context as _, Result};

use console::{Key, Term};

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
	frame.render_widget(format!("foo bar {:?}", Instant::now()), frame.area());
}
