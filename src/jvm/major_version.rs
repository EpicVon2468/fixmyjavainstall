use std::fmt::{Display, Formatter};
use std::str::FromStr;

use clap::builder::TypedValueParser;
use clap::error::ErrorKind;
use clap::{Arg, Command, Error};

#[derive(Clone, PartialEq)]
pub enum MajorVersion {
	Number(u32),
	/// Latest
	Latest,
	/// Long Term Support
	LTS,
}

impl Display for MajorVersion {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				MajorVersion::Number(value) => value.to_string(),
				MajorVersion::Latest => "latest".into(),
				MajorVersion::LTS => "lts".into(),
			}
		)
	}
}

#[derive(Clone)]
pub struct MajorVersionParser;

impl TypedValueParser for MajorVersionParser {
	type Value = MajorVersion;

	fn parse_ref(
		&self,
		_cmd: &Command,
		_arg: Option<&Arg>,
		value: &std::ffi::OsStr,
	) -> Result<Self::Value, Error> {
		value.to_str().unwrap().to_lowercase().as_str().parse()
	}
}

// https://stackoverflow.com/questions/73658377/how-to-have-number-or-string-as-a-cli-argument-in-clap
impl FromStr for MajorVersion {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.parse::<u32>() {
			Ok(num) => Ok(MajorVersion::Number(num)),
			Err(_) => match s {
				"latest" => Ok(MajorVersion::Latest),
				"lts" => Ok(MajorVersion::LTS),
				_ => Err(Error::new(ErrorKind::InvalidValue)),
			},
		}
	}
}
