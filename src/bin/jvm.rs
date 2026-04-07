use fuji::subcommand_entrypoint;

/// `fuji-jvm`
pub fn main() {
	subcommand_entrypoint(&["manage".into(), "jvm".into()]);
}