#![cfg(feature = "tui")]

use std::cell::Cell;
use std::time::Duration;

use anyhow::Result;

use ratatui::crossterm::event::{Event, KeyCode, poll, read};
use ratatui::layout::{Constraint, Layout, Margin};
use ratatui::widgets::{Block, BorderType};
use ratatui::{DefaultTerminal, Frame};

use crate::tui::page::Page;
use crate::tui::page::jvm::JVMPage;
use crate::tui::render_title;
use crate::tui::tab::Tab;

pub struct FujiApp {
	pub page: Cell<Box<dyn Page>>,
	pub event: Option<Event>,
	pub prev_event: Option<Event>,
}

impl FujiApp {
	pub fn new() -> Self {
		Self {
			page: Cell::new(Box::new(JVMPage { tab: Tab::Foo })),
			event: None,
			prev_event: None,
		}
	}

	pub fn key_pressed(&self, prev: bool, validate: &dyn Fn(KeyCode) -> bool) -> bool {
		let event = if prev { &self.prev_event } else { &self.event }.as_ref();
		if let Some(&Event::Key(key_event)) = event {
			validate(key_event.code)
		} else {
			false
		}
	}

	pub fn should_exit(&self) -> bool {
		self.key_pressed(false, &|key| {
			self.key_pressed(true, &|key| matches!(key, KeyCode::Char(':')))
				&& matches!(key, KeyCode::Char('q'))
		})
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
		render_title(frame, title);
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

	const BORDER: Block<'static> = Block::bordered().border_type(BorderType::Rounded);
}
