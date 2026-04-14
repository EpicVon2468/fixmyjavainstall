//! The `fuji-jvm` binary.  Equivalent to running `fuji manage jvm`.
use anyhow::Result;

use fuji::alias_entrypoint;

/// `fuji-jvm`
pub fn main() -> Result<()> {
	alias_entrypoint(&["manage".into(), "jvm".into()])
}
