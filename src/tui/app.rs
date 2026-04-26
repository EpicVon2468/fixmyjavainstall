#![cfg(feature = "tui")]

use std::cell::UnsafeCell;
use std::mem::replace;

use anyhow::{Context as _, Result};

use console::Key;

use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::prelude::Line;
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Span;
use ratatui::widgets::{Block, BorderType};
use ratatui::{DefaultTerminal, Frame, try_init};

use crate::tui::page::Page;
use crate::tui::page::jvm::JVMPage;
use crate::tui::tab::Tab;
use crate::{compiler_unreachable, matches_many};

pub struct FujiApp {
	page: UnsafeCell<Box<dyn Page>>,
	event: Option<Key>,
	prev_event: Option<Key>,
}

/// Encapsulation (Rust is the only language where doing this can be for a good reason).
impl FujiApp {
	pub fn new() -> Self {
		Self {
			page: UnsafeCell::new(Box::new(JVMPage { tab: Tab::Foo })),
			event: None,
			prev_event: None,
		}
	}

	pub fn run() -> Result<()> {
		Self::new().main(try_init().context("Couldn't initialise ratatui!")?)
	}

	fn page(&self) -> *mut Box<dyn Page> {
		self.page.get()
	}

	fn get_page(&self) -> Box<dyn Page> {
		// SAFETY: todo
		unsafe { self.page().read() }
	}

	fn set_page(&mut self, value: Box<dyn Page>) {
		// SAFETY: todo
		unsafe {
			self.page().write(value);
		}
	}
}

/// Rendering.
impl FujiApp {
	pub fn main(mut self, mut terminal: DefaultTerminal) -> Result<()> {
		loop {
			self.propagate_events();
			terminal.draw(|frame: &mut Frame| self.render(frame))?;
			self.prev_event = replace(&mut self.event, Self::update()?);
			if self.should_exit() {
				break Ok(());
			};
		}
	}

	/// See also: [`Component::propagate_events`][`crate::tui::component::Component::propagate_events`].
	fn propagate_events(&mut self) {
		let mut page: Box<dyn Page> = self.get_page();
		if page.propagate_events(self) {
			self.event.take();
			self.prev_event.take();
		};
		self.set_page(page);
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
			let mut page: Box<dyn Page> = self.get_page();
			page.render(frame, area.inner(Margin::new(1, 1)), self);
			self.set_page(page);
		};
	}

	pub const BORDER: Block<'static> = Block::bordered().border_type(BorderType::Rounded);
}

/// Help section.
impl FujiApp {
	fn render_help(frame: &mut Frame, area: Rect) {
		let [top, bottom] = area.layout(&Self::help_layout());
		Self::render_help_top_row(frame, top);
		Self::render_help_bottom_row(frame, bottom);
	}

	fn help_layout() -> Layout {
		Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)])
	}

	fn render_help_top_row(frame: &mut Frame, area: Rect) {
		let [quit, back] = area.layout(&Self::help_top_row_layout());
		Self::help_entry(frame, quit, ":q", "Quit");
		Self::help_entry(frame, back, "Esc", "Back");
	}

	fn help_top_row_layout() -> Layout {
		Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).spacing(1)
	}

	fn render_help_bottom_row(frame: &mut Frame, area: Rect) {
		let [select] = area.layout(&Self::help_bottom_row_layout());
		Self::help_entry(frame, select, "Enter", "Select");
	}

	fn help_bottom_row_layout() -> Layout {
		Layout::horizontal([Constraint::Fill(1)]).spacing(1)
	}

	fn help_entry(frame: &mut Frame, area: Rect, key: &str, action: &str) {
		frame.render_widget(
			Line::from_iter([
				Span::styled(key, Self::HELP_KEY),
				Span::raw(format!(" {action}")),
			]),
			area,
		);
	}

	pub const HELP_KEY: Style = Style::new().on_white().black();
}

/// Keybinds.
impl FujiApp {
	fn check_key(&self, prev: bool, validate: &dyn Fn(&Key) -> bool) -> bool {
		let event: Option<&Key> = if prev { &self.prev_event } else { &self.event }.as_ref();
		event.is_some_and(validate)
	}

	#[allow(clippy::needless_pass_by_value)]
	fn key_down(&self, prev: bool, key: Key) -> bool {
		self.check_key(prev, &|event: &Key| *event == key)
	}

	pub fn is_key_down(&self, key: Key) -> bool {
		self.key_down(false, key)
	}

	pub fn was_key_down(&self, key: Key) -> bool {
		self.key_down(true, key)
	}

	/// Whether the [`FujiApp`] should exit.
	///
	/// Returns: if the sequence `:q` was pressed.
	pub fn should_exit(&self) -> bool {
		self.was_key_down(Key::Char(':')) && self.is_key_down(Key::Char('q'))
	}
}

/// Man-made horrors beyond your comprehension.
impl FujiApp {
	pub fn update() -> Result<Option<Key>> {
		use ratatui::crossterm::event::{Event, KeyCode, ModifierKeyCode, read};

		let event: Event = read()?;
		if !event.is_key() {
			return Ok(None);
		};
		let Event::Key(key_event): Event = event else {
			compiler_unreachable!();
		};

		if matches_many!(
			key_event.code,
			KeyCode::F(_),
			KeyCode::Null,
			KeyCode::CapsLock,
			KeyCode::ScrollLock,
			KeyCode::NumLock,
			KeyCode::PrintScreen,
			KeyCode::Pause,
			KeyCode::Menu,
			KeyCode::KeypadBegin,
			KeyCode::Media(_),
			KeyCode::Modifier(modifier) if matches_many!(
				modifier,
				ModifierKeyCode::LeftControl,
				ModifierKeyCode::RightControl,
				ModifierKeyCode::LeftSuper,
				ModifierKeyCode::RightSuper,
				ModifierKeyCode::LeftHyper,
				ModifierKeyCode::RightHyper,
				ModifierKeyCode::LeftMeta,
				ModifierKeyCode::RightMeta,
			),
		) {
			return Ok(None);
		};

		let value: Key = match key_event.code {
			KeyCode::Backspace => Key::Backspace,
			KeyCode::Enter => Key::Enter,
			KeyCode::Left => Key::ArrowLeft,
			KeyCode::Right => Key::ArrowRight,
			KeyCode::Up => Key::ArrowUp,
			KeyCode::Down => Key::ArrowDown,
			KeyCode::Home => Key::Home,
			KeyCode::End => Key::End,
			KeyCode::PageUp => Key::PageUp,
			KeyCode::PageDown => Key::PageDown,
			KeyCode::Tab => Key::Tab,
			KeyCode::BackTab => Key::BackTab,
			KeyCode::Delete => Key::Del,
			KeyCode::Insert => Key::Insert,
			KeyCode::Char(val) => Key::Char(val),
			KeyCode::Esc => Key::Escape,
			KeyCode::Modifier(modifier) => match modifier {
				ModifierKeyCode::LeftShift
				| ModifierKeyCode::RightShift
				| ModifierKeyCode::IsoLevel3Shift
				| ModifierKeyCode::IsoLevel5Shift => Key::Shift,
				ModifierKeyCode::LeftAlt | ModifierKeyCode::RightAlt => Key::Alt,
				_ => compiler_unreachable!(),
			},
			_ => compiler_unreachable!(),
		};

		Ok(Some(value))
	}
}
