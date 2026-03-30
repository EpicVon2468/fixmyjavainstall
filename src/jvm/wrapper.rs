use crate::jvm::manage_jvm::Feature;

pub fn generate_wrapper(features: &Vec<Feature>) -> String {
	let mut result: String = String::with_capacity(500);
	result.push_str("#! /usr/bin/env sh\n");
	if features.contains(&Feature::DCEVM) {
		result.push_str("
# Dynamic Code Evolution Virtual Machine (enhanced runtime class redefinition) – https://ssw.jku.at/dcevm/
ADDITIONAL_JVM_ARGS=\"-XX:+AllowEnhancedClassRedefinition $ADDITIONAL_JVM_ARGS\"\n\
		");
	};
	if features.contains(&Feature::JEP519) {
		result.push_str("
# JDK Enhancement Proposal 519 (Compact Object Headers) – https://openjdk.org/jeps/519
ADDITIONAL_JVM_ARGS=\"-XX:+UseCompactObjectHeaders $ADDITIONAL_JVM_ARGS\"\n\
		");
	};
	let mut requires_vulkan: bool = false;
	if features.contains(&Feature::WLToolkit) {
		result.push_str("
# Wayland support (requires Vulkan) – https://wiki.openjdk.org/spaces/wakefield/pages/77693134/Pure+Wayland+toolkit+prototype
ADDITIONAL_JVM_ARGS=\"-Dawt.tookit.name=WLToolkit $ADDITIONAL_JVM_ARGS\"\n\
		");
		requires_vulkan = true;
	};
	if features.contains(&Feature::OpenGL) {
		if requires_vulkan {
			panic!("Vulkan required for WLToolkit, but OpenGL was also explicitly requested.  Resolve incompatible args and try again.");
		};
		result.push_str("
# OpenGL for AWT/Swing.  This has been bundled in OpenJDK for a long time, but isn't on by default
ADDITIONAL_JVM_ARGS=\"-Dsun.java2d.opengl=true $ADDITIONAL_JVM_ARGS\"\n\
		");
	};
	if requires_vulkan || features.contains(&Feature::Vulkan) {
		result.push_str("
# Vulkan for AWT/Swing
ADDITIONAL_JVM_ARGS=\"-Dsun.java2d.vulkan=true -Dsun.java2d.vulkan.accelsd=false $ADDITIONAL_JVM_ARGS\"\n\
		");
	};
	result.push_str("\nexec /opt/fuji/jvm/latest/bin/java \"$ADDITIONAL_JVM_ARGS\" \"$@\"");
	result
}