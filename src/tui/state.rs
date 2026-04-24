#![cfg(feature = "tui")]

use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Offset, Rect};
use ratatui::widgets::{Block, Paragraph, Tabs, Widget};

pub struct State {
	pub tab: Tab,
}

pub enum Tab {
	Foo,
	Bar,
	Baz,
}

impl Widget for &Tab {
	fn render(self, area: Rect, buf: &mut Buffer) {
		#[allow(unreachable_patterns)]
		let text: &str = match *self {
			Tab::Foo => "Something something foo",
			Tab::Bar => "Something something bar",
			Tab::Baz => "Something something baz",
			_ => unreachable!(),
		};
		let paragraph: Paragraph = Paragraph::new(text)
			.alignment(Alignment::Center)
			.block(Block::bordered());
		paragraph.render(area, buf);

		#[allow(clippy::as_conversions)]
		let tabs: Tabs = Tabs::new(Tab::value_names().to_owned())
			.select(self.ordinal() as usize)
			.padding(" ", " ");
		tabs.render(area + Offset::new(1, 0), buf);
	}
}

impl Tab {
	pub const fn value_names() -> &'static [&'static str] {
		&["Foo", "Bar", "Baz"]
	}

	pub const fn ordinal(&self) -> u32 {
		match *self {
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

	pub const fn shift_left(&self) -> Self {
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

	pub const fn shift_right(&self) -> Self {
		let mut ord: u32 = self.ordinal() + 1;
		if ord > Self::last().ordinal() {
			ord = 0;
		};
		Self::get(ord).unwrap()
	}
}
