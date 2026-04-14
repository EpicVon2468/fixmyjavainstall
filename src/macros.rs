#[macro_export]
macro_rules! wrong_cmd {
	($name:ident) => {
		#[rustfmt::skip]
		return anyhow::Result::Err(
			std::io::Error::new(
				std::io::ErrorKind::InvalidInput,
				concat!("Function ", stringify!($name), "() had wrong parameter!"),
			).into(),
		);
	};
}

#[macro_export]
macro_rules! wait_and_check_status {
	($child:ident) => {
		$crate::wait_and_check_status!($child, "command");
	};
	($child:ident, $name:literal) => {
		$crate::wait_and_check_status!($child, $name, 1);
	};
	($child:ident, $name:literal, $substitute_code:literal) => {{
		use std::process::ExitStatus;

		let status: ExitStatus = $child.wait().context(concat!($name, " never started?"))?;
		$crate::check_status!(status, $name, $substitute_code);
	}};
}

#[macro_export]
macro_rules! check_status {
	($status:ident) => {
		$crate::check_status!($status, "command");
	};
	($status:ident, $name:literal) => {
		$crate::check_status!($status, $name, 1);
	};
	($status:ident, $name:literal, $substitute_code:literal) => {{
		if (!$status.success()) {
			return anyhow::Result::Err(anyhow::anyhow!(format!(
				concat!($name, " failed with exit code: {}"),
				$status.code().unwrap_or($substitute_code)
			)));
		};
	}};
}

#[macro_export]
macro_rules! fuji_value_enum {
	($ty:ident, match {$($variant:pat => $string:literal),*,}) => {
		#[automatically_derived]
		impl Default for $ty {
			fn default() -> Self {
				Self::SYSTEM
			}
		}

		#[automatically_derived]
		impl From<$ty> for clap::builder::OsStr {
			fn from(value: $ty) -> Self {
				value.to_string().into()
			}
		}

		#[automatically_derived]
		impl std::fmt::Display for $ty {
			#[allow(unreachable_code, unreachable_patterns)]
			fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
				write!(
					fmt,
					"{}",
					match *self {
						$($variant => $string,)*
						_ => panic!("Not implemented!"),
					}
				)
			}
		}
	};
}
