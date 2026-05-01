#![cfg(feature = "tui")]
pub mod home;
pub mod jvm;

use crate::tui::app::FujiApp;
use crate::tui::component::Component;

pub const trait Page: Component<Return = ()> {
	fn title(&self) -> Option<&'static str> {
		None
	}

	fn propagate_page_events(&mut self, app: &FujiApp) -> (bool, Option<Box<dyn Page>>);
}
