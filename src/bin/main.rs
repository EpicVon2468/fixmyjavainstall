use anyhow::Result;

use clap::Parser;

use fuji::cli::FujiArgs;
use fuji::entrypoint;

/// `fuji`
pub fn main() -> Result<()> {
	entrypoint(FujiArgs::parse())
}
