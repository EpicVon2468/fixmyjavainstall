#![cfg(feature = "tui")]
pub mod home;
pub mod jvm;

use mtc::Component;

use crate::tui::app::FujiApp;

pub const trait Page: Component<FujiApp> {
	fn title(&self) -> &'static str {
		"A JVM & Kotlin Management Utility"
	}

	fn propagate_page_events(
		&mut self,
		app: &FujiApp,
	) -> anyhow::Result<(bool, Option<Box<dyn Page>>)>;
}
