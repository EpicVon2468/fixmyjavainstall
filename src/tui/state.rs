#![cfg(feature = "tui")]

use ratatui::crossterm::event::Event;

use crate::tui::tab::Tab;

pub struct FujiState {
	pub page: Page,
	pub event: Option<Event>,
}

pub enum Page {
	Home,
	JVM { tab: Tab },
}

impl FujiState {
	pub const fn new() -> Self {
		Self {
			page: Page::JVM { tab: Tab::Foo },
			event: None,
		}
	}
}
