#[macro_export]
macro_rules! wrong_cmd {
	($name:ident) => {
		return anyhow::Result::Err(std::io::Error::new(
			std::io::ErrorKind::InvalidInput,
			concat!("Function ", stringify!($name), "() had wrong parameter!"),
		).into());
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
	($child:ident, $name:literal, $substitute_code:literal) => {
		let exit_status: std::process::ExitStatus = $child.wait().expect(concat!($name, " never started?"));
		$crate::check_status!(exit_status, $name, $substitute_code);
	};
}

#[macro_export]
macro_rules! check_status {
	($status:ident) => {
		$crate::check_status!($status, "command");
	};
	($status:ident, $name:literal) => {
		$crate::check_status!($status, $name, 1);
	};
	($status:ident, $name:literal, $substitute_code:literal) => {
		if (!$status.success()) {
			return anyhow::Result::Err(anyhow::anyhow!(format!(
				concat!($name, " failed with exit code: {}"),
				$status.code().unwrap_or($substitute_code)
			)));
		};
	};
}