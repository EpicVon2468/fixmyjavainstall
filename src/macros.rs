#[macro_export]
macro_rules! wrong_cmd {
    ($name:ident) => {
		return Err(
			Error::new(
				ErrorKind::InvalidData,
				concat!("Function ", stringify!($name), "() had wrong parameter!")
			)
		);
	};
}