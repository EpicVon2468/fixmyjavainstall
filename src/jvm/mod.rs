//! The modules responsible for (un)installing & managing the Java Virtual Machine.
//!
//! The modules are laid out as related to the following groups:
//!
//! <details><summary>Java Development Kits:</summary>
//!
//! * [`jdk`] – The enumeration of supported JDKs.
//! * [`jdk_generic`] – I don't even know at this point.
//! * [`jdk_java_se`] – The download handler for [`Java Platform, Standard Edition`][`jdk::JDK::JavaSE`].
//! * [`jdk_jbr`] – The download handler for [`JetBrains Runtime`][`jdk::JDK::JBR`].
//! * [`jdk_liberica`] – The download handler for [`Liberica`][`jdk::JDK::Liberica`].
//! * [`jdk_temurin`] – The download handler for [`Eclipse Temurin`][`jdk::JDK::Temurin`].
//! </details>
#[cfg(target_os = "linux")]
pub mod desktop;
pub mod install;
pub mod jdk;
pub mod jdk_generic;
pub mod jdk_java_se;
pub mod jdk_jbr;
pub mod jdk_liberica;
pub mod jdk_temurin;
pub mod major_version;
pub mod manage_jvm;
pub mod wrapper;