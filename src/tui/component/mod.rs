#![cfg(feature = "tui")]
pub mod list;
pub mod logo;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::tui::app::FujiApp;

pub trait Component {
	type Return;

	/// Pre-[`render`][`Component::render`] input/logic checks.
	///
	/// Returns whether the [`Component`] has 'consumed' the input event (and thusly, whether any parent components should be allowed to process input).
	///
	/// Implementations are expected to follow a bottom-up hierarchy of evaluation, where a [`Component`] may only process events if all its children have first been processed, and none have returned `true`.
	#[allow(unused_variables)]
	fn propagate_events(&mut self, app: &FujiApp) -> bool {
		false
	}

	fn render(&self, frame: &mut Frame, area: Rect, app: &FujiApp) -> Self::Return;
}
