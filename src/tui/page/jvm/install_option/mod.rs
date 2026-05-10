#![cfg(feature = "tui")]
pub mod arch_option;
pub mod feature_option;
pub mod jvm_option;
pub mod method_option;
#[cfg(feature = "multi-os")]
pub mod os_option;

use mtc::Component;

use crate::tui::app::FujiApp;

pub const trait InstallOption: Component<FujiApp> {
	// FIXME: clippy stopped marking this as unused?
	fn is_completed(&self) -> bool {
		true
	}

	fn tab_name(&self) -> &str;
}

#[macro_export]
macro_rules! install_option {
	($name:ident, $underlying_type:ty) => {
		install_option!($name, $underlying_type, false);
	};
	($name:ident, $underlying_type:ty, $multi_select:literal) => {
		#[automatically_derived]
		impl Default for $name<'_> {
			fn default() -> Self {
				Self {
					list: List::from(<$underlying_type>::value_variants(), $multi_select),
				}
			}
		}

		#[automatically_derived]
		impl Component<$crate::tui::app::FujiApp> for $name<'_> {
			fn propagate_events(&mut self, app: &FujiApp) -> bool {
				self.list.propagate_events(app)
			}

			fn render(&self, frame: &mut Frame, area: Rect, app: &FujiApp) {
				self.list.render(frame, area, app);
			}
		}
	};
}
