pub mod home;
pub mod jvm;

use crate::tui::component::Component;

pub trait Page: Component {}
