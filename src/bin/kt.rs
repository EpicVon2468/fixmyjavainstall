//! The `fuji-kt` binary.  Equivalent to running `fuji manage kotlin`.
use anyhow::Result;

use fuji::subcommand_entrypoint;

/// `fuji-kt`
pub fn main() -> Result<()> {
	subcommand_entrypoint(&["manage".into(), "kotlin".into()])
}