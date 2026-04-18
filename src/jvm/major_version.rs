//! An enumeration data structure for representing major JVM versions.
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use clap::builder::{PossibleValue, TypedValueParser};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{Arg, Command, Error};

/// The major version of a JVM.
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Default)]
pub enum MajorVersion {
	/// Some arbitrary numeric version.
	Number(u32),
	/// The latest version.
	Latest,
	/// The latest Long Term Support version.
	#[default]
	LTS,
}

impl FujiValueEnum for MajorVersion {
	#[allow(unreachable_patterns)]
	fn to_possible_value(&self) -> Option<PossibleValue> {
		match *self {
			Self::Number(_) => PossibleValue::new("[0..4_294_967_295]")
				.help("Some arbitrary numeric version")
				.into(),
			Self::Latest => PossibleValue::new("latest")
				.help("The latest version")
				.into(),
			Self::LTS => PossibleValue::new("lts")
				.help("The latest Long Term Support version")
				.into(),
			_ => None,
		}
	}

	fn variants() -> &'static [Self] {
		&[Self::Number(0), Self::Latest, Self::LTS]
	}
}

pub trait FujiValueEnum: FromStr<Err = String> + 'static {
	fn possible_values() -> impl Iterator<Item = PossibleValue> {
		Self::variants().iter().filter_map(Self::to_possible_value)
	}

	#[allow(unreachable_patterns)]
	fn to_possible_value(&self) -> Option<PossibleValue>;

	#[must_use]
	fn variants() -> &'static [Self];
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

#[derive(Default, Clone)]
pub struct FujiValueEnumParser<T: FujiValueEnum>(std::marker::PhantomData<T>);

impl<T: FujiValueEnum> FujiValueEnumParser<T> {
	pub fn parse_impl<P: TypedValueParser>(
		cmd: &Command,
		arg: Option<&Arg>,
		value: &OsStr,
	) -> Result<P::Value, Error>
	where
		P::Value: FujiValueEnum, {
		let result: Result<P::Value, String> = value.to_str().unwrap().to_lowercase().parse();
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
						MajorVersion::possible_values()
							.map(|val: PossibleValue| val.get_name().to_owned())
							.collect(),
					),
				);
				Err(error)
			},
			Ok,
		)
	}
}

#[macro_export]
macro_rules! fuji_value_enum_parser {
	($name:ty) => {
		impl TypedValueParser for FujiValueEnumParser<$name> {
			type Value = $name;

			fn parse_ref(
				&self,
				cmd: &Command,
				arg: Option<&Arg>,
				value: &OsStr,
			) -> Result<Self::Value, Error> {
				Self::parse_impl::<Self>(cmd, arg, value)
			}

			fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
				Some(Box::new(<$name>::possible_values()))
			}
		}
	};
}

fuji_value_enum_parser!(MajorVersion);

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
