#![cfg(feature = "tui")]
pub mod app;
pub mod component;
pub mod page;

use ratatui::style::Style;

pub const INVERTED: Style = Style::new().reversed();

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
