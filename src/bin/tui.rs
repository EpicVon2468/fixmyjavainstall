//! The `fuji-tui` binary.
use anyhow::Result;

/// `fuji-tui`
pub fn main() -> Result<()> {
	cfg_select! {
		feature = "tui" => fuji::tui::main(),
		_ => Ok(()),
	}
}
