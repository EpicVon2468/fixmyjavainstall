use serde::{Deserialize as Deserialise, Serialize as Serialise};

#[derive(Serialise, Deserialise)]
pub struct KtVersion {
	pub version: String,
}