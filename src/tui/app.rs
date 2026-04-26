#![cfg(feature = "tui")]

use std::cell::Cell;

use anyhow::{Context as _, Result};

use ratatui::crossterm::event::{Event, KeyCode, read};
use ratatui::layout::{Constraint, Layout, Margin, Offset, Rect};
use ratatui::prelude::Line;
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Span;
use ratatui::widgets::{Block, BorderType};
use ratatui::{DefaultTerminal, Frame, try_init};

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

	pub fn run() -> Result<()> {
		Self::new().main(try_init().context("Couldn't initialise ratatui!")?)
	}

	pub fn main(mut self, mut terminal: DefaultTerminal) -> Result<()> {
		loop {
			terminal.draw(|frame: &mut Frame| self.render(frame))?;
			self.prev_event = self.event.replace(read()?);
			if self.should_exit() {
				break Ok(());
			};
		}
	}

	fn app_layout() -> Layout {
		Layout::vertical([
			Constraint::Length(1),
			Constraint::Fill(1),
			Constraint::Length(2),
		])
		.spacing(1)
	}

	fn render(&mut self, frame: &mut Frame) {
		let [title, body, help] = frame.area().layout(&Self::app_layout());
		Self::render_title(frame, title);
		self.render_body(frame, body);
		Self::render_help(frame, help);
	}

	fn render_title(frame: &mut Frame, area: Rect) {
		let title: Line = Line::from("Fix Ur Java Install – A JVM & Kotlin Management Utility.")
			.centered()
			.bold();
		frame.render_widget(title, area);
	}

	fn render_body(&mut self, frame: &mut Frame, area: Rect) {
		// Content box
		frame.render_widget(Self::BORDER, area);
		{
			let ptr: *mut Box<dyn Page> = self.page.as_ptr();
			// SAFETY: todo
			let mut page: Box<dyn Page> = unsafe { ptr.read() };
			page.render(frame, area.inner(Margin::new(1, 1)), self);
			// SAFETY: todo
			unsafe {
				ptr.write(page);
			};
		};
	}

	pub const BORDER: Block<'static> = Block::bordered().border_type(BorderType::Rounded);
}

/// Help section.
#[allow(non_snake_case)]
impl FujiApp {
	fn render_help(frame: &mut Frame, area: Rect) {
		let area: Rect = area + Offset::new(1, -1);
		let [help__quit, help__back] = area.layout(&Self::help_layout());
		Self::render_help__quit(frame, help__quit);
		Self::render_help__back(frame, help__back);
	}

	fn help_layout() -> Layout {
		Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).spacing(1)
	}

	fn render_help__quit(frame: &mut Frame, area: Rect) {
		frame.render_widget(Self::help_entry(":q", "Quit"), area);
	}

	fn render_help__back(frame: &mut Frame, area: Rect) {
		frame.render_widget(Self::help_entry("Esc", "Back"), area);
	}

	fn help_entry<'a>(key: &'a str, action: &str) -> Line<'a> {
		Line::from_iter([
			Span::styled(key, Self::HELP_KEY),
			Span::raw(format!(" {action}")),
		])
	}

	pub const HELP_KEY: Style = Style::new().on_white().black();
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
