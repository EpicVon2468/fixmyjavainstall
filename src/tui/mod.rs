#![cfg(feature = "tui")]
mod app;
mod page;
mod tab;

use anyhow::{Context as _, Result};

use ratatui::{DefaultTerminal, try_init, try_restore};

use crate::tui::app::FujiApp;

// https://github.com/ratatui/ratatui/blob/main/examples/concepts/state/src/bin/stateful-widget.rs
// https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples
// https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/block.rs
// https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/tabs.rs
pub fn main() -> Result<()> {
	let terminal: DefaultTerminal = try_init().context("Couldn't initialise ratatui!")?;
	let result: Result<()> = FujiApp::new().main(terminal);
	let _ = try_restore();
	result
}
