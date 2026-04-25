#![cfg(feature = "tui")]

use std::time::Duration;

use anyhow::Result;

use ratatui::crossterm::event::{Event, KeyCode, poll, read};
use ratatui::layout::{Constraint, Layout};
use ratatui::{DefaultTerminal, Frame};

use crate::tui::render_title;
use crate::tui::tab::Tab;

pub struct FujiApp {
	pub page: Page,
	pub event: Option<Event>,
}

pub enum Page {
	Home,
	JVM { tab: Tab },
}

impl FujiApp {
	pub const fn new() -> Self {
		Self {
			page: Page::JVM { tab: Tab::Foo },
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
		match self.page {
			Page::Home => {},
			Page::JVM { mut tab } => {
				frame.render_stateful_widget(&mut tab, body, self);
				self.page = Page::JVM { tab };
			},
		};
	}
}
