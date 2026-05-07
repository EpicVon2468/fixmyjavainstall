#![cfg(feature = "tui")]
pub mod app;
pub mod component;
pub mod page;

use ratatui::style::Style;

pub const INVERTED: Style = Style::new().reversed();

#[macro_export]
macro_rules! static_layout {
	($value:expr $(,)?) => {
		$crate::static_layout!(LAYOUT, $value);
	};
	($name:ident, $value:expr $(,)?) => {
		$crate::static_anything!($name, Layout, $value);
	};
}

#[macro_export]
macro_rules! static_anything {
	($name:ident, $ty:ty, $value:expr $(,)?) => {
		static $name: std::sync::LazyLock<$ty> = std::sync::LazyLock::new(|| $value);
	};
}

#[macro_export]
macro_rules! centred_horizontally {
	($area:expr, $width:expr $(,)?) => {
		$area.centered_horizontally(ratatui::layout::Constraint::Length($crate::to_u16!($width)))
	};
}

#[macro_export]
macro_rules! to_u16 {
	($value:expr $(,)?) => {
		u16::try_from($value).unwrap_or(u16::MAX)
	};
}
