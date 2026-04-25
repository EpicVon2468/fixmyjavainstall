#![cfg(feature = "tui")]

use std::cell::Cell;
use std::time::Duration;

use anyhow::Result;

use ratatui::crossterm::event::{Event, KeyCode, poll, read};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::{DefaultTerminal, Frame};

use crate::tui::render_title;
use crate::tui::tab::Tab;

pub struct FujiApp {
	pub page: Cell<Box<dyn Page>>,
	pub event: Option<Event>,
}

pub trait Page {
	fn render(&mut self, frame: &mut Frame, area: Rect, app: &mut FujiApp);
}

struct JVMPage {
	tab: Tab,
}

impl Page for JVMPage {
	fn render(&mut self, frame: &mut Frame, area: Rect, app: &mut FujiApp) {
		frame.render_stateful_widget(&mut self.tab, area, app);
	}
}

impl FujiApp {
	pub fn new() -> Self {
		Self {
			page: Cell::new(Box::new(JVMPage { tab: Tab::Foo })),
			event: None,
		}
	}

	pub fn main(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
		loop {
			terminal.draw(|frame: &mut Frame| self.render(frame))?;
			if !poll(Duration::from_millis(0))? {
				if self.event.is_some() {
					self.event.take();
				};
				continue;
			};
			let event: Event = read()?;
			if let Event::Key(key_event) = event
				&& key_event.code == KeyCode::Char('q')
			{
				break Ok(());
			};
			self.event.replace(event);
		}
	}

	fn render(&mut self, frame: &mut Frame) {
		let layout: Layout =
			Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
		let [title, body] = frame.area().layout(&layout);
		render_title(frame, title);
		let ptr: *mut Box<dyn Page> = self.page.as_ptr();
		// SAFETY: todo
		let mut page: Box<dyn Page> = unsafe { ptr.read() };
		page.render(frame, body, self);
		// SAFETY: todo
		unsafe {
			ptr.write(page);
		};
	}
}
