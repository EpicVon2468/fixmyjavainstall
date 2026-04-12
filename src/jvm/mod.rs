//! The modules responsible for (un)installing & managing the Java Virtual Machine.
//!
//! The modules are laid out as related to the following groups:
//!
//! <details><summary>Java Virtual Machines:</summary>
//!
//! * [`jvm`] – The enumeration of supported JVM builds/vendors.
//! * [`jvm_generic`] – I don't even know at this point.
//! * [`jvm_java_se`] – The download handler for [`Java Platform, Standard Edition`][`jvm::JVM::JavaSE`].
//! * [`jvm_jbr`] – The download handler for [`JetBrains Runtime`][`jvm::JVM::JBR`].
//! * [`jvm_liberica`] – The download handler for [`Liberica`][`jvm::JVM::Liberica`].
//! * [`jvm_temurin`] – The download handler for [`Eclipse Temurin`][`jvm::JVM::Temurin`].
//! </details>
pub mod cmd_install;
pub mod cmd_preset;
#[cfg(target_os = "linux")]
pub mod desktop;
#[allow(clippy::module_inception)]
pub mod jvm;
pub mod jvm_generic;
pub mod jvm_java_se;
pub mod jvm_jbr;
pub mod jvm_liberica;
pub mod jvm_temurin;
pub mod major_version;
pub mod manage_jvm;
pub mod wrapper;