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
	dbg!(size_of::<FujiApp>());
	dbg!(size_of::<page::home::HomePage>());
	dbg!(size_of::<page::jvm::JVMPage>());
	let result: Result<()> = FujiApp::run();
	let _ = try_restore();
	result
}

#[macro_export]
macro_rules! static_layout {
	($value:expr) => {
		static_layout!(LAYOUT, $value);
	};
	($name:ident, $value:expr) => {
		static $name: std::sync::LazyLock<ratatui::layout::Layout> =
			std::sync::LazyLock::new(|| $value);
	};
}

pub const INVERTED: Style = Style::new().reversed();
