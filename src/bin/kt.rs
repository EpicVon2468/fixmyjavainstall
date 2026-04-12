//! The `fuji-kt` binary.  Equivalent to running `fuji manage kotlin`.
use anyhow::Result;

use fuji::alias_entrypoint;

/// `fuji-kt`
pub fn main() -> Result<()> {
	alias_entrypoint(&["manage".into(), "kotlin".into()])
}