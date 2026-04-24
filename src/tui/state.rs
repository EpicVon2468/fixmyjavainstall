#![cfg(feature = "tui")]

use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{HorizontalAlignment, Margin, Offset, Rect};
use ratatui::widgets::{Block, Paragraph, StatefulWidget, Tabs, Widget as _};

pub struct FujiState {
	pub tab: Tab,
	pub event: Option<Event>,
}

#[derive(Copy, Clone)]
pub enum Tab {
	Foo,
	Bar,
	Baz,
}

impl StatefulWidget for Tab {
	type State = FujiState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
		StatefulWidget::render(&self, area, buf, state);
	}
}

impl StatefulWidget for &Tab {
	type State = FujiState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut FujiState) {
		state.event.clone().inspect(|event: &Event| {
			if let Event::Key(key_event) = *event {
				match key_event.code {
					KeyCode::Left => {
						state.tab.shift_self_left();
					},
					KeyCode::Right => {
						state.tab.shift_self_right();
					},
					_ => (),
				};
			}
		});

		#[allow(unreachable_patterns)]
		let render_tab: fn(Rect, &mut Buffer, &mut FujiState) = match *self {
			Tab::Foo => Tab::render_foo,
			Tab::Bar => Tab::render_bar,
			Tab::Baz => Tab::render_baz,
			_ => unreachable!(),
		};
		Block::bordered().render(area, buf);
		render_tab(area.inner(Margin::new(1, 1)), buf, state);

		#[allow(clippy::as_conversions)]
		let tabs: Tabs = Tabs::new(Tab::value_names().to_owned())
			.select(self.ordinal() as usize)
			.padding(" ", " ")
			.divider("#");
		tabs.render(area + Offset::new(1, 0), buf);
	}
}

impl Tab {
	fn render_foo(area: Rect, buf: &mut Buffer, state: &mut FujiState) {
		let paragraph: Paragraph = Paragraph::new("text").alignment(HorizontalAlignment::Center);
		paragraph.render(area, buf);
	}

	fn render_bar(area: Rect, buf: &mut Buffer, state: &mut FujiState) {}

	fn render_baz(area: Rect, buf: &mut Buffer, state: &mut FujiState) {}

	pub const fn value_names() -> &'static [&'static str] {
		&["Foo", "Bar", "Baz"]
	}

	pub const fn ordinal(self) -> u32 {
		match self {
			Self::Foo => 0,
			Self::Bar => 1,
			Self::Baz => 2,
		}
	}

	pub const fn get(ordinal: u32) -> Option<Self> {
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

	pub const fn shift_left(self) -> Self {
		let mut ord: i32 = self.ordinal().cast_signed() - 1;
		if ord < 0 {
			ord = Self::last().ordinal().cast_signed();
		};
		Self::get(ord.cast_unsigned()).unwrap()
	}

	pub const fn shift_self_right(&mut self) -> &mut Self {
		*self = self.shift_right();
		self
	}

	pub const fn shift_right(self) -> Self {
		let mut ord: u32 = self.ordinal() + 1;
		if ord > Self::last().ordinal() {
			ord = 0;
		};
		Self::get(ord).unwrap()
	}
}
