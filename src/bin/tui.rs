//! The `fuji-tui` binary.
use anyhow::Result;

use fuji::tui::app::FujiApp;

/// `fuji-tui`
pub fn main() -> Result<()> {
	FujiApp::run()
}
