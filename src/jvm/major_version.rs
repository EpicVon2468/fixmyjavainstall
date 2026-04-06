use std::ffi::{OsString, OsStr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use clap::builder::{PossibleValue, TypedValueParser};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{Arg, Command, Error};

/// The major version of a JVM
#[derive(Clone, PartialEq)]
pub enum MajorVersion {
	/// Some arbitrary numeric version
	Number(u32),
	/// The latest version
	Latest,
	/// The latest Long Term Support version
	LTS,
}

impl MajorVersion {

	fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			MajorVersion::Number(_) => PossibleValue::new("[0..4_294_967_295]")
				.help("Some arbitrary numeric version"),
			MajorVersion::Latest => PossibleValue::new("latest")
				.help("The latest version"),
			MajorVersion::LTS => PossibleValue::new("lts")
				.help("The latest Long Term Support version"),
		}.into()
	}
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

impl MajorVersionParser {

	fn possible_values() -> impl Iterator<Item = PossibleValue> {
		[
			MajorVersion::Number(0),
			MajorVersion::Latest,
			MajorVersion::LTS,
		].iter().filter_map(MajorVersion::to_possible_value)
	}
}

impl TypedValueParser for MajorVersionParser {

	type Value = MajorVersion;

	fn parse_ref(
		&self,
		cmd: &Command,
		arg: Option<&Arg>,
		value: &OsStr,
	) -> Result<Self::Value, Error> {
		self.parse(cmd, arg, value.to_owned())
	}

	fn parse(
		&self,
		cmd: &Command,
		arg: Option<&Arg>,
		value: OsString,
	) -> Result<Self::Value, Error> {
		let result: Result<MajorVersion, String> = value
			.to_str()
			.unwrap()
			.to_lowercase()
			.parse();
		match result {
			Ok(version) => Ok(version),
			Err(invalid_value) => {
				let mut error: Error = Error::new(ErrorKind::InvalidValue).with_cmd(cmd);
				if let Some(argument) = arg {
					error.insert(
						ContextKind::InvalidArg,
						ContextValue::String(argument.to_string()),
					);
				};
				error.insert(
					ContextKind::InvalidValue,
					ContextValue::String(invalid_value),
				);
				error.insert(
					ContextKind::ValidValue,
					ContextValue::Strings(
						Self::possible_values()
							.map(|v: PossibleValue| v.get_name().to_owned())
							.collect()
					),
				);
				Err(error)
			}
		}
	}

	fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
		Some(Box::new(Self::possible_values()))
	}
}

// https://stackoverflow.com/questions/73658377/how-to-have-number-or-string-as-a-cli-argument-in-clap
impl FromStr for MajorVersion {

	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.parse() {
			Ok(num) => Ok(MajorVersion::Number(num)),
			Err(_) => match s {
				"latest" => Ok(MajorVersion::Latest),
				"lts" => Ok(MajorVersion::LTS),
				_ => Err(s.into()),
			},
		}
	}
}