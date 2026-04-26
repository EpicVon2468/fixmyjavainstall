use console::Key;

use ratatui::Frame;
use ratatui::layout::{HorizontalAlignment, Offset, Rect};
use ratatui::widgets::{Paragraph, Tabs};

use crate::tui::app::FujiApp;
use crate::tui::component::Component;

pub enum Tab {
	Foo,
	Bar,
	Baz,
}

impl Component for Tab {
	type Return = ();

	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		if app.is_key_down(Key::ArrowLeft) {
			self.shift_self_left();
			return true;
		};
		if app.is_key_down(Key::ArrowRight) {
			self.shift_self_right();
			return true;
		};
		false
	}

	fn render(&mut self, frame: &mut Frame, area: Rect, app: &mut FujiApp) -> Self::Return {
		let render_content: fn(&mut Frame, Rect, &mut FujiApp) = match *self {
			Self::Foo => Self::render_foo,
			Self::Bar => Self::render_bar,
			Self::Baz => Self::render_baz,
		};
		render_content(frame, area, app);

		let tabs: Tabs = Tabs::new(Self::value_names().to_owned())
			.select(self.ordinal())
			.padding(" ", " ")
			.divider("#");
		frame.render_widget(tabs, area - Offset::new(0, 1));
	}
}

/// Rendering.
impl Tab {
	fn render_foo(frame: &mut Frame, area: Rect, _app: &mut FujiApp) {
		let paragraph: Paragraph = Paragraph::new("text").alignment(HorizontalAlignment::Center);
		frame.render_widget(paragraph, area);
	}

	fn render_bar(frame: &mut Frame, area: Rect, app: &mut FujiApp) {}

	fn render_baz(frame: &mut Frame, area: Rect, app: &mut FujiApp) {}
}

/// Ordinal information.
impl Tab {
	pub const fn value_names() -> &'static [&'static str] {
		&["Foo", "Bar", "Baz"]
	}

	pub const fn ordinal(&self) -> usize {
		match *self {
			Self::Foo => 0,
			Self::Bar => 1,
			Self::Baz => 2,
		}
	}

	pub const fn get(ordinal: usize) -> Option<Self> {
		match ordinal {
			0 => Some(Self::Foo),
			1 => Some(Self::Bar),
			2 => Some(Self::Baz),
			_ => None,
		}
	}

	pub const LAST: Self = Self::Baz;
	pub const LAST_ORDINAL: usize = Self::LAST.ordinal();

	pub const fn shift_self_left(&mut self) -> &mut Self {
		*self = self.shift_left();
		self
	}

	pub const fn shift_left(&self) -> Self {
		let mut ord: isize = self.ordinal().cast_signed().saturating_sub(1);
		if ord < 0 {
			ord = Self::LAST_ORDINAL.cast_signed();
		};
		Self::get(ord.cast_unsigned()).unwrap()
	}

	pub const fn shift_self_right(&mut self) -> &mut Self {
		*self = self.shift_right();
		self
	}

	pub const fn shift_right(&self) -> Self {
		let mut ord: usize = self.ordinal().saturating_add(1);
		if ord > Self::LAST_ORDINAL {
			ord = 0;
		};
		Self::get(ord).unwrap()
	}
}
