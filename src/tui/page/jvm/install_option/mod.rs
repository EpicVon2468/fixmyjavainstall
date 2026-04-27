pub mod jvm_option;

use crate::tui::component::Component;

pub trait InstallOption: Component<Return = ()> {
	fn is_completed(&self) -> bool {
		true
	}
}
