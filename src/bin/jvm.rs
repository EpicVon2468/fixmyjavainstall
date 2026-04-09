//! The `fuji-jvm` binary.  Equivalent to running `fuji manage jvm`.
use anyhow::Result;

use fuji::subcommand_entrypoint;

/// `fuji-jvm`
pub fn main() -> Result<()> {
	subcommand_entrypoint(&["manage".into(), "jvm".into()])
}