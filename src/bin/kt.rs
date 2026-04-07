#![doc = "fuji-kt"]
use fuji::subcommand_entrypoint;

/// `fuji-kt`
pub fn main() {
	subcommand_entrypoint(&["manage".into(), "kotlin".into()]);
}