#![cfg(feature = "tui")]
use clap::ValueEnum as _;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::install_option;
use crate::jvm::feature::Feature;
use crate::tui::app::FujiApp;
use crate::tui::component::Component;
use crate::tui::component::list::{List, ListEntry};
use crate::tui::page::jvm::install_option::InstallOption;

impl const ListEntry for Feature {
	fn name(&self) -> &str {
		match *self {
			Self::Minimal => "Minimal",
			Self::DCEVM => "Dynamic Code Evolution Virtual Machine",
			Self::JEP519 => "Compact Object Headers",
			Self::Wayland => "Wayland support for AWT/Swing",
			Self::OpenGL => "OpenGL for AWT/Swing",
			Self::Metal => "Metal for AWT/Swing",
			Self::Vulkan => "Vulkan for AWT/Swing",
			Self::JCEF => "Java Chromium Embedded Framework",
			Self::Native => "Native access",
			Self::Unsafe => "Unsafe access",
			Self::Mutate => "Mutate access",
			Self::FontFix => "Font rendering fixes",
			Self::NVIDIA => "NVIDIA rendering fixes",
			Self::MUSL => "MUSL support",
			Self::Kotlin => "Kotlin",
		}
	}
}

pub struct FeatureOption<'a> {
	list: List<'a>,
}

install_option!(FeatureOption, Feature, true);

impl const InstallOption for FeatureOption<'_> {
	fn tab_name(&self) -> &'static str {
		"Features"
	}
}
