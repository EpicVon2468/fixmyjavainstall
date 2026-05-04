#![cfg(feature = "tui")]
mod app;
mod component;
mod page;

use anyhow::Result;

use ratatui::style::Style;
use ratatui::try_restore;

use crate::tui::app::FujiApp;

// https://github.com/ratatui/ratatui/blob/main/examples/concepts/state/src/bin/stateful-widget.rs
// https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples
// https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/block.rs
// https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/tabs.rs
pub fn main() -> Result<()> {
	dbg!(size_of::<*mut Box<dyn page::Page>>());
	dbg!(size_of::<std::cell::UnsafeCell<Box<dyn page::Page>>>());
	dbg!(size_of::<FujiApp>());
	let result: Result<()> = FujiApp::run();
	let _ = try_restore();
	result
}

pub const INVERTED: Style = Style::new().reversed();
