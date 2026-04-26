#![cfg(feature = "tui")]

use std::cell::Cell;
use std::time::Duration;

use anyhow::Result;

use ratatui::crossterm::event::{Event, KeyCode, poll, read};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::prelude::Line;
use ratatui::style::Stylize as _;
use ratatui::widgets::{Block, BorderType};
use ratatui::{DefaultTerminal, Frame};

use crate::tui::page::Page;
use crate::tui::page::jvm::JVMPage;
use crate::tui::tab::Tab;

pub struct FujiApp {
	pub page: Cell<Box<dyn Page>>,
	pub event: Option<Event>,
	pub prev_event: Option<Event>,
}

/// Rendering.
impl FujiApp {
	pub fn new() -> Self {
		Self {
			page: Cell::new(Box::new(JVMPage { tab: Tab::Foo })),
			event: None,
			prev_event: None,
		}
	}

	pub fn main(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
		loop {
			terminal.draw(|frame: &mut Frame| self.render(frame))?;
			if !poll(Duration::from_millis(0))? {
				if self.event.is_some() {
					self.prev_event = self.event.take();
				};
				continue;
			};
			self.event.replace(read()?);
			if self.should_exit() {
				break Ok(());
			};
		}
	}

	fn render(&mut self, frame: &mut Frame) {
		let layout: Layout = Layout::vertical([
			Constraint::Length(1),
			Constraint::Fill(1),
			Constraint::Length(2),
		])
		.spacing(1);
		let [title, body, help] = frame.area().layout(&layout);
		Self::render_title(frame, title);
		frame.render_widget(Self::BORDER, body);
		let ptr: *mut Box<dyn Page> = self.page.as_ptr();
		// SAFETY: todo
		let mut page: Box<dyn Page> = unsafe { ptr.read() };
		page.render(frame, body.inner(Margin::new(1, 1)), self);
		// SAFETY: todo
		unsafe {
			ptr.write(page);
		};
	}

	fn render_title(frame: &mut Frame, area: Rect) {
		let title: Line = Line::from("Fix Ur Java Install – A JVM & Kotlin Management Utility.")
			.centered()
			.bold();
		frame.render_widget(title, area);
	}

	const BORDER: Block<'static> = Block::bordered().border_type(BorderType::Rounded);
}

/// Keybinds.
impl FujiApp {
	fn check_key(&self, prev: bool, validate: &dyn Fn(KeyCode) -> bool) -> bool {
		let event: &Option<Event> = if prev { &self.prev_event } else { &self.event };
		if let Some(Event::Key(key_event)) = *event {
			validate(key_event.code)
		} else {
			false
		}
	}

	fn key_down(&self, prev: bool, key: KeyCode) -> bool {
		self.check_key(prev, &|event: KeyCode| event == key)
	}

	pub fn is_key_down(&self, key: KeyCode) -> bool {
		self.key_down(false, key)
	}

	pub fn was_key_down(&self, key: KeyCode) -> bool {
		self.key_down(true, key)
	}

	/// Whether the [`FujiApp`] should exit.
	///
	/// Returns: if the sequence `:q` was pressed.
	pub fn should_exit(&self) -> bool {
		self.was_key_down(KeyCode::Char(':')) && self.is_key_down(KeyCode::Char('q'))
	}
}
