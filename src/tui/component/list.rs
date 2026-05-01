#![cfg(feature = "tui")]
use ratatui::Frame;
use ratatui::crossterm::event::KeyCode;
use ratatui::layout::{Offset, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};

use crate::tui::INVERTED;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;

// Based on ratatui-widget's Widget of the same name, except remade for my needs + Component trait
#[derive(Default)]
pub struct List<'a> {
	items: Vec<Span<'a>>,
	selected: usize,
	confirmed: Vec<usize>,
	confirmed_prefix: String,
	unconfirmed_prefix: String,
	selected_prefix: String,
	unselected_prefix: String,
	selected_style: Style,
	confirmed_style: Style,
	multi_confirm: bool,
}

pub const trait ListEntry {
	fn name(&self) -> &str;

	fn description(&self) -> Option<&str> {
		None
	}
}

#[allow(unused)]
impl<'a> List<'a> {
	pub fn new<T>(items: T, multi_confirm: bool) -> Self
	where
		T: IntoIterator,
		T::Item: Into<Span<'a>>, {
		Self {
			items: items.into_iter().map(Into::into).collect(),
			confirmed_prefix: "[*]".into(),
			unconfirmed_prefix: "[ ]".into(),
			selected_style: INVERTED,
			confirmed_style: INVERTED,
			selected_prefix: ">".into(),
			unselected_prefix: " ".into(),
			multi_confirm,
			..Default::default()
		}
	}

	pub fn from<T: ListEntry>(items: &'a [T], multi_confirm: bool) -> Self {
		Self::new(items.iter().map(T::name), multi_confirm)
	}

	pub fn confirmed_prefix(&mut self, value: String) -> &mut Self {
		self.confirmed_prefix = value;
		self
	}

	pub fn unconfirmed_prefix(&mut self, value: String) -> &mut Self {
		self.unconfirmed_prefix = value;
		self
	}

	pub fn selected_prefix(&mut self, value: String) -> &mut Self {
		self.selected_prefix = value;
		self
	}

	pub fn unselected_prefix(&mut self, value: String) -> &mut Self {
		self.unselected_prefix = value;
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
			self.selected = self.selected.saturating_sub(1);
		};
		self
	}

	pub const fn select_next(&mut self) -> &mut Self {
		if self.selected >= self.last_index() {
			// loop around
			self.selected = 0;
		} else {
			self.selected = self.selected.saturating_add(1);
		};
		self
	}

	pub fn is_confirmed(&self, value: usize) -> bool {
		self.confirmed.contains(&value)
	}

	pub fn toggle_confirmed(&mut self, value: usize) {
		if self.multi_confirm {
			self.toggle_confirmed_multi_confirm(value);
		} else {
			self.toggle_confirmed_single_confirm(value);
		};
	}

	fn toggle_confirmed_multi_confirm(&mut self, value: usize) {
		if let Some(index) = self.confirmed.iter().position(|val: &usize| *val == value) {
			// if the value already exists, remove it
			self.confirmed.swap_remove(index);
		} else {
			self.confirmed.push(value);
		};
	}

	fn toggle_confirmed_single_confirm(&mut self, value: usize) {
		if self.confirmed.first() == Some(&value) {
			// if the value already exists, remove it
			self.confirmed.clear();
		} else {
			self.confirmed = vec![value];
		};
	}
}

impl Component for List<'_> {
	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		if app.is_key_down(KeyCode::Up) {
			self.select_prev();
			return true;
		};
		if app.is_key_down(KeyCode::Down) {
			self.select_next();
			return true;
		};
		if app.is_key_down(KeyCode::Enter) {
			self.toggle_confirmed(self.selected);
			return true;
		};
		false
	}

	// TODO: handle scroll if items is longer than area's height
	fn render(&self, frame: &mut Frame, area: Rect, _app: &FujiApp) -> Self::Return {
		let mut area: Rect = area;
		for (index, item) in self.items.iter().enumerate() {
			let is_confirmed: bool = self.is_confirmed(index);
			let is_selected: bool = self.selected == index;
			let line: Line = Line::styled(
				format!(
					"{} {} {item}",
					if is_selected {
						&self.selected_prefix
					} else {
						&self.unselected_prefix
					},
					if is_confirmed {
						&self.confirmed_prefix
					} else {
						&self.unconfirmed_prefix
					},
				),
				if is_confirmed {
					self.confirmed_style
				} else if is_selected {
					self.selected_style
				} else {
					Style::new()
				},
			);
			frame.render_widget(line, area);
			area += Offset::new(0, 1);
		}
	}
}
