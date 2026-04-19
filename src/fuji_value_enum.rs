use std::ffi::OsStr;
use std::str::FromStr;

use clap::builder::{PossibleValue, TypedValueParser};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{Arg, Command, Error};

pub trait FujiValueEnum: FromStr<Err = String> + 'static {
	fn possible_values() -> impl Iterator<Item = PossibleValue> {
		Self::variants().iter().filter_map(Self::to_possible_value)
	}

	#[allow(unreachable_patterns)]
	fn to_possible_value(&self) -> Option<PossibleValue>;

	#[must_use]
	fn variants() -> &'static [Self];
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
						T::possible_values()
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
