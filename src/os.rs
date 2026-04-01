use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum OS {
	Linux,
	/// macOS by Apple
	OSX,
	/// Windows by Microslop
	Windows,
}