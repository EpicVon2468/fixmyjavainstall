use console::Key;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize as _;
use ratatui::text::{Line, Text};

use crate::LONG_VERSION;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::page::Page;
use crate::tui::page::jvm::JVMPage;
use crate::tui::tab::Tab;

pub struct HomePage;

impl HomePage {
	pub const fn new() -> Self {
		Self {}
	}

	fn layout() -> Layout {
		Layout::vertical([Constraint::Length(7), Constraint::Fill(1)]).spacing(1)
	}

	fn render_logo_and_about(frame: &mut Frame, area: Rect) {
		let [top, bottom] = area.layout(&Self::logo_and_about_layout());
		frame.render_widget(Text::raw(Self::LOGO).centered().bold().light_blue(), top);
		frame.render_widget(Line::from(LONG_VERSION).centered(), bottom);
	}

	fn logo_and_about_layout() -> Layout {
		Layout::vertical([Constraint::Fill(1), Constraint::Length(1)])
	}

	pub const LOGO: &'static str = include_str!("../../../doc/logo.txt");
}

impl Page for HomePage {
	fn propagate_page_events(&mut self, app: &FujiApp) -> (bool, Option<Box<dyn Page>>) {
		let consumed: bool = self.propagate_events(app);
		if consumed {
			return (true, None);
		};
		if app.is_key_down(Key::Enter) {
			(true, Some(Box::new(JVMPage::new(Tab::Foo))))
		} else {
			(false, None)
		}
	}
}

impl Component for HomePage {
	type Return = ();

	fn propagate_events(&mut self, _app: &FujiApp) -> bool {
		false
	}

	fn render(&self, frame: &mut Frame, area: Rect, _app: &mut FujiApp) -> Self::Return {
		let [top, _bottom] = area.layout(&Self::layout());
		Self::render_logo_and_about(frame, top);
	}
}
