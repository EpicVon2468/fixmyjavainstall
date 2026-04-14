//! An enumeration data structure for representing major JVM versions
use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use clap::builder::{PossibleValue, TypedValueParser};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{Arg, Command, Error};

/// The major version of a JVM
#[derive(Clone, PartialEq, Eq)]
pub enum MajorVersion {
	/// Some arbitrary numeric version
	Number(u32),
	/// The latest version
	Latest,
	/// The latest Long Term Support version
	LTS,
}

impl MajorVersion {
	#[allow(
		clippy::unnecessary_wraps,
		reason = "False positive; This function is called in a filter_map call, and must be Option<_>.  Bad clippy!"
	)]
	fn to_possible_value(&self) -> Option<PossibleValue> {
		Some(match *self {
			Self::Number(_) =>
				PossibleValue::new("[0..4_294_967_295]").help("Some arbitrary numeric version"),
			Self::Latest => PossibleValue::new("latest").help("The latest version"),
			Self::LTS => PossibleValue::new("lts").help("The latest Long Term Support version"),
		})
	}
}

impl Display for MajorVersion {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			fmt,
			"{}",
			match *self {
				Self::Number(value) => value.to_string(),
				Self::Latest => "latest".into(),
				Self::LTS => "lts".into(),
			}
		)
	}
}

#[derive(Clone, Default)]
pub struct MajorVersionParser;

impl MajorVersionParser {
	#[inline]
	#[must_use]
	pub const fn new() -> Self {
		Self {}
	}

	pub fn possible_values() -> impl Iterator<Item = PossibleValue> {
		Self::variants()
			.iter()
			.filter_map(MajorVersion::to_possible_value)
	}

	#[inline]
	#[must_use]
	pub const fn variants() -> &'static [MajorVersion] {
		&[
			MajorVersion::Number(0),
			MajorVersion::Latest,
			MajorVersion::LTS,
		]
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
		let result: Result<MajorVersion, String> = value.to_str().unwrap().to_lowercase().parse();
		result.map_or_else(
			|invalid_value: String| {
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
							.map(|val: PossibleValue| val.get_name().to_owned())
							.collect(),
					),
				);
				Err(error)
			},
			|version: MajorVersion| Ok(version),
		)
	}

	fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
		Some(Box::new(Self::possible_values()))
	}
}

// https://stackoverflow.com/questions/73658377/how-to-have-number-or-string-as-a-cli-argument-in-clap
impl FromStr for MajorVersion {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.parse().map_or_else(
			|_| match s {
				"latest" => Ok(Self::Latest),
				"lts" => Ok(Self::LTS),
				_ => Err(s.into()),
			},
			|num: u32| Ok(Self::Number(num)),
		)
	}
}
