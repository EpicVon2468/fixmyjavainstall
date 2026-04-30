#![cfg(feature = "tui")]
use std::cell::UnsafeCell;

use anyhow::{Context as _, Result};

use console::Key;

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::style::Stylize as _;
use ratatui::widgets::{Block, BorderType, Padding};
use ratatui::{DefaultTerminal, Frame, try_init};

use crate::tui::component::Component as _;
use crate::tui::component::help::HelpSection;
use crate::tui::page::Page;
use crate::tui::page::home::HomePage;
use crate::{compiler_unreachable, matches_many};

pub struct FujiApp {
	page: UnsafeCell<Box<dyn Page>>,
	event: Option<Key>,
	prev_event: Option<Key>,
	layout: Layout,
	help_section: HelpSection,
}

impl Default for FujiApp {
	fn default() -> Self {
		Self {
			page: UnsafeCell::new(Box::new(HomePage::default())),
			event: None,
			prev_event: None,
			layout: Layout::vertical([
				Constraint::Length(1),
				Constraint::Fill(1),
				Constraint::Length(2),
			]),
			help_section: Default::default(),
		}
	}
}

/// Encapsulation (Rust is the only language where doing this can be for a good reason).
impl FujiApp {
	pub fn run() -> Result<()> {
		Self::default().main(try_init().context("Couldn't initialise ratatui!")?)
	}

	fn page(&self) -> *mut Box<dyn Page> {
		self.page.get()
	}

	/// # Safety
	///
	/// You must always call [`Self::set_page`] before the value returned by this method goes out-of-scope (as such, this function cannot not be called safely without `&mut self` being available).
	///
	/// The value which is passed to the [`Self::set_page`] is irrelevant – the only requirement is that _some value_ is restored via [`Self::set_page`] before the value returned by this method goes out-of-scope.
	///
	/// Failure to do so means that the underlying value of [`Self::page`] will be automagically [`dropped`][`std::mem::drop`] by Rust.
	///
	/// This can lead to:
	///
	/// - Undefined behaviour.
	/// - Use-after-free bugs.
	/// - Segmentation faults.
	/// - Memory leaks.
	/// - Double `free()`s.
	unsafe fn get_page(&self) -> Box<dyn Page> {
		// SAFETY:
		// Problem(s):
		// - Pointers are unsafe.
		// Excuse(s):
		// - This function is only invoked by trusted callers in a safe manner.
		// - Both this function and the underlying struct field are private and cannot be unexpectedly mutated.
		unsafe { self.page().read() }
	}

	fn set_page(&mut self, value: Box<dyn Page>) {
		// SAFETY:
		// Problem(s):
		// - Pointers are unsafe.
		// Excuse(s):
		// - This function is only invoked by trusted callers in a safe manner.
		// - Both this function and the underlying struct field are private and cannot be unexpectedly mutated.
		// - Mutations of [`Self::page`] are not inherently unsafe, and may be performed without consequence.
		unsafe {
			self.page().write(value);
		}
	}
}

/// Logic.
impl FujiApp {
	fn main(mut self, mut terminal: DefaultTerminal) -> Result<()> {
		loop {
			self.propagate_events();
			terminal.draw(|frame: &mut Frame| self.render(frame))?;
			self.prev_event = std::mem::replace(&mut self.event, Self::update()?);
			if self.should_exit() {
				break Ok(());
			};
		}
	}

	/// See also: [`Component::propagate_events`][`crate::tui::component::Component::propagate_events`].
	fn propagate_events(&mut self) {
		// SAFETY:
		// Problem(s):
		// - If this value goes out of scope, undefined behaviour occurs (see [`Self::get_page`]).
		// Excuse(s):
		// - Before the end of scope, a call to [`Self::set_page`] is made, meaning that the contract of [`Self::get_page`] is never violated.
		let mut page: Box<dyn Page> = unsafe { self.get_page() };
		let (consumed, new_page): (bool, Option<Box<dyn Page>>) = page.propagate_page_events(self);
		if consumed {
			self.event.take();
			self.prev_event.take();
		};
		self.set_page(new_page.unwrap_or(page));
	}
}

/// Rendering.
impl FujiApp {
	fn render(&mut self, frame: &mut Frame) {
		let [title, body, help] = frame.area().layout(&self.layout);
		self.render_title(frame, title);
		self.render_body(frame, body);
		self.help_section.render(frame, help, self);
	}

	fn render_title(&mut self, frame: &mut Frame, area: Rect) {
		let title: Line = Line::from(self.get_title()).centered().bold().light_blue();
		frame.render_widget(title, area);
	}

	fn get_title(&mut self) -> String {
		// SAFETY:
		// Problem(s):
		// - If this value goes out of scope, undefined behaviour occurs (see [`Self::get_page`]).
		// Excuse(s):
		// - Before the end of scope, a call to [`Self::set_page`] is made, meaning that the contract of [`Self::get_page`] is never violated.
		let page: Box<dyn Page> = unsafe { self.get_page() };
		let title: String = format!(
			"Fix Ur Java Install – {}",
			page.title().unwrap_or("A JVM & Kotlin Management Utility"),
		);
		self.set_page(page);
		title
	}

	fn render_body(&mut self, frame: &mut Frame, area: Rect) {
		// Content box
		frame.render_widget(Self::BORDER, area);
		{
			// SAFETY:
			// Problem(s):
			// - If this value goes out of scope, undefined behaviour occurs (see [`Self::get_page`]).
			// Excuse(s):
			// - Before the end of scope, a call to [`Self::set_page`] is made, meaning that the contract of [`Self::get_page`] is never violated.
			let page: Box<dyn Page> = unsafe { self.get_page() };
			page.render(frame, Self::BORDER.inner(area), self);
			self.set_page(page);
		};
	}

	pub const BORDER: Block<'static> = Block::bordered()
		.padding(Padding::uniform(1))
		.border_type(BorderType::Rounded);
}

/// Keybinds.
impl FujiApp {
	const fn get_event(&self, prev: bool) -> Option<&Key> {
		if prev { &self.prev_event } else { &self.event }.as_ref()
	}

	#[allow(clippy::needless_pass_by_value)]
	fn key_down(&self, prev: bool, key: Key) -> bool {
		matches!(self.get_event(prev), Some(event_key) if *event_key == key)
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
