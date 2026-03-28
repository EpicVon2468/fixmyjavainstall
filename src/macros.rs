#[macro_export]
macro_rules! wrong_cmd {
    ($name:ident) => {
		return core::result::Result::Err(
			std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				concat!("Function ", stringify!($name), "() had wrong parameter!")
			)
		);
	};
}

#[macro_export]
macro_rules! check_status {
	($status:ident) => {
		check_status!($status, "command")
	};
    ($status:ident, $name:literal) => {
		check_status!($status, $name, 1)
	};
	($status:ident, $name:literal, $substituteCode:literal) => {
		if (!$status.success()) {
			return core::result::Result::Err(
				std::io::Error::other(
					format!(concat!($name, " failed with exit code: {}"), $status.code().unwrap_or($substituteCode))
				)
			);
		};
	};
}