#![cfg(feature = "tui")]
use console::Key;
use ratatui::Frame;
use ratatui::layout::{Offset, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};

use crate::tui::app::FujiApp;
use crate::tui::component::Component;

// TODO: abstract this out to allow single-select and multi-select
// Based on ratatui-widget's Widget of the same name, except remade for my needs + Component trait
#[derive(Default)]
pub struct List<'a> {
	items: Vec<Span<'a>>,
	selected: usize,
	confirmed: Option<usize>,
	confirmed_prefix: String,
	unconfirmed_prefix: String,
	selected_style: Style,
	confirmed_style: Style,
}

pub trait ListEntry {
	fn name(&self) -> &'static str;

	fn description(&self) -> Option<&'static str> {
		None
	}
}

impl<T: ListEntry> From<&[T]> for List<'static> {
	fn from(value: &[T]) -> Self {
		Self::new(value.iter().map(ListEntry::name))
	}
}

#[allow(unused)]
impl<'a> List<'a> {
	pub fn new<T>(items: T) -> Self
	where
		T: IntoIterator,
		T::Item: Into<Span<'a>>, {
		Self {
			items: items.into_iter().map(Into::into).collect(),
			confirmed_prefix: "[*]".into(),
			unconfirmed_prefix: "[ ]".into(),
			selected_style: Self::DEFAULT_STYLE,
			confirmed_style: Self::DEFAULT_STYLE,
			..Default::default()
		}
	}

	const DEFAULT_STYLE: Style = Style::new().black().on_white();

	pub fn confirmed_prefix(&mut self, value: String) -> &mut Self {
		self.confirmed_prefix = value;
		self
	}

	pub fn unconfirmed_prefix(&mut self, value: String) -> &mut Self {
		self.unconfirmed_prefix = value;
		self
	}

	pub const fn selected_style(&mut self, value: Style) -> &mut Self {
		self.selected_style = value;
		self
	}

	pub const fn confirmed_style(&mut self, value: Style) -> &mut Self {
		self.confirmed_style = value;
		self
	}

	pub const fn selected(&self) -> usize {
		self.selected
	}

	pub const fn last_index(&self) -> usize {
		self.items.len().saturating_sub(1)
	}

	pub const fn select_prev(&mut self) -> &mut Self {
		if self.selected == 0 {
			// loop around
			self.selected = self.last_index();
		} else {
			self.selected -= 1;
		};
		self
	}

	pub const fn select_next(&mut self) -> &mut Self {
		if self.selected == self.last_index() {
			// loop around
			self.selected = 0;
		} else {
			self.selected += 1;
		};
		self
	}

	pub fn is_confirmed(&self, value: usize) -> bool {
		self.confirmed.is_some_and(|val: usize| val == value)
	}
}

impl Component for List<'_> {
	type Return = ();

	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		if app.is_key_down(Key::ArrowUp) {
			self.select_prev();
			return true;
		};
		if app.is_key_down(Key::ArrowDown) {
			self.select_next();
			return true;
		};
		if app.is_key_down(Key::Enter) {
			// toggle confirmed
			if self.is_confirmed(self.selected) {
				self.confirmed = None;
			} else {
				self.confirmed = Some(self.selected);
			};
			return true;
		};
		false
	}

	// TODO: handle scroll if items is longer than area's height
	fn render(&self, frame: &mut Frame, area: Rect, _app: &FujiApp) -> Self::Return {
		let mut area: Rect = area;
		for (index, item) in self.items.iter().enumerate() {
			let is_confirmed: bool = self.is_confirmed(index);
			let mut line: Line = Line::default().patch_style(if is_confirmed {
				self.confirmed_style
			} else if self.selected == index {
				self.selected_style
			} else {
				Style::new()
			});
			line.push_span(if is_confirmed {
				&self.confirmed_prefix
			} else {
				&self.unconfirmed_prefix
			});
			line.push_span(" ");
			line.push_span(item.clone());
			frame.render_widget(line, area);
			area += Offset::new(0, 1);
		}
	}
}
