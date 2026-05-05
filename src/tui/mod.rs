#![cfg(feature = "tui")]
pub mod app;
pub mod component;
pub mod page;

use ratatui::style::Style;

pub const INVERTED: Style = Style::new().reversed();

#[macro_export]
macro_rules! static_layout {
	($value:expr $(,)?) => {
		static_layout!(LAYOUT, $value);
	};
	($name:ident, $value:expr $(,)?) => {
		static_anything!($name, Layout, $value);
	};
}

#[macro_export]
macro_rules! static_anything {
	($name:ident, $ty:ty, $value:expr $(,)?) => {
		static $name: std::sync::LazyLock<$ty> = std::sync::LazyLock::new(|| $value);
	};
}
