use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{HorizontalAlignment, Offset, Rect};
use ratatui::prelude::{StatefulWidget, Widget as _};
use ratatui::widgets::{Paragraph, Tabs};

use crate::tui::app::FujiApp;

pub enum Tab {
	Foo,
	Bar,
	Baz,
}

impl StatefulWidget for &mut Tab {
	type State = FujiApp;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut FujiApp) {
		state.event.as_ref().inspect(|event: &&Event| {
			if let Event::Key(key_event) = **event {
				match key_event.code {
					KeyCode::Left => {
						self.shift_self_left();
					},
					KeyCode::Right => {
						self.shift_self_right();
					},
					_ => (),
				};
			}
		});

		#[allow(unreachable_patterns)]
		let render_tab: fn(Rect, &mut Buffer, &mut FujiApp) = match *self {
			Tab::Foo => Tab::render_foo,
			Tab::Bar => Tab::render_bar,
			Tab::Baz => Tab::render_baz,
			_ => unreachable!(),
		};
		render_tab(area, buf, state);

		let tabs: Tabs = Tabs::new(Tab::value_names().to_owned())
			.select(self.ordinal())
			.padding(" ", " ")
			.divider("#");
		tabs.render(area - Offset::new(0, 1), buf);
	}
}

impl Tab {
	fn render_foo(area: Rect, buf: &mut Buffer, _state: &mut FujiApp) {
		let paragraph: Paragraph = Paragraph::new("text").alignment(HorizontalAlignment::Center);
		paragraph.render(area, buf);
	}

	fn render_bar(area: Rect, buf: &mut Buffer, state: &mut FujiApp) {}

	fn render_baz(area: Rect, buf: &mut Buffer, state: &mut FujiApp) {}

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

	pub const fn first() -> Self {
		Self::get(0).unwrap()
	}

	pub const fn last() -> Self {
		Self::Baz
	}

	pub const fn shift_self_left(&mut self) -> &mut Self {
		*self = self.shift_left();
		self
	}

	pub const fn shift_left(&self) -> Self {
		let mut ord: isize = self.ordinal().cast_signed().saturating_sub(1);
		if ord < 0 {
			ord = Self::last().ordinal().cast_signed();
		};
		Self::get(ord.cast_unsigned()).unwrap()
	}

	pub const fn shift_self_right(&mut self) -> &mut Self {
		*self = self.shift_right();
		self
	}

	pub const fn shift_right(&self) -> Self {
		let mut ord: usize = self.ordinal().saturating_add(1);
		if ord > Self::last().ordinal() {
			ord = 0;
		};
		Self::get(ord).unwrap()
	}
}
