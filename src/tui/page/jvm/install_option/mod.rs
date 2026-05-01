#![cfg(feature = "tui")]
pub mod arch_option;
pub mod jvm_option;
pub mod os_option;

use crate::tui::component::Component;

pub const trait InstallOption: Component<Return = ()> {
	fn is_completed(&self) -> bool {
		true
	}

	fn tab_name(&self) -> &'static str;
}
