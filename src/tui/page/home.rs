use console::Key;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};

use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::component::logo::FujiLogo;
use crate::tui::page::Page;
use crate::tui::page::jvm::JVMPage;

#[derive(Default)]
pub struct HomePage {
	logo: FujiLogo,
}

impl HomePage {
	#[allow(unused)]
	pub const fn new(logo: FujiLogo) -> Self {
		Self { logo }
	}

	fn layout() -> Layout {
		Layout::vertical([Constraint::Length(7), Constraint::Fill(1)])
	}
}

impl Page for HomePage {
	fn propagate_page_events(&mut self, app: &FujiApp) -> (bool, Option<Box<dyn Page>>) {
		let consumed: bool = self.propagate_events(app);
		if consumed {
			return (true, None);
		};
		if app.is_key_down(Key::Enter) {
			(true, Some(Box::new(JVMPage::default())))
		} else {
			(false, None)
		}
	}
}

impl Component for HomePage {
	type Return = ();

	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		self.logo.propagate_events(app)
	}

	fn render(&self, frame: &mut Frame, area: Rect, app: &FujiApp) -> Self::Return {
		let [top, _bottom] = area.layout(&Self::layout());
		self.logo.render(frame, top, app);
	}
}
