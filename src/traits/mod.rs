//! # Traits
//!
//! This module provides a growing collection of interface traits. Each trait
//! captures a common pattern or capability, making it easier to write generic
//! code and adapters between libraries.
//!
//! ## Current submodules
//!
//! - [`conv`] – Conversion traits (`Convertable`, `ConvertableRef`, `ConvertableMut`)
//!   and their unsafe counterparts.
//! - [`data`] – Data‑oriented traits, such as `AbsFlags` for bitwise flag types.
//! - [`osdev`] – Traits for resource management (e.g. `AbsPageFrameManager`), which
//!   are useful not only in kernels but in any system that manages indexed resources.
//! - [`serde`] – Re‑exports of `serde::Serialize` and `serde::Deserialize` for
//!   convenience, making it easy to derive or implement these traits for your own
//!   types without adding an explicit `serde` dependency.
//! - [`log`] – Re‑export of the `log::Log` trait, allowing you to implement
//!   custom loggers without adding a direct dependency on the `log` crate.
//!
//! ## Future expansion
//!
//! The number of submodules will grow over time to cover more patterns:
//! iterators, async operations, fallible conversions, etc. The goal is to provide
//! a comprehensive set of “standard interfaces” that different crates can rely on.
//!
//! ## Usage
//!
//! Import the traits you need, either directly from their submodules or via
//! the re‑exports:
//!
//! ```
//! use faces::traits::{Convertable, AbsFlags, Serialize, Deserialize, Log};
//! ```

pub mod osdev;
pub mod conv;
pub mod data;
pub mod serde;
pub mod log;

pub use osdev::*;
pub use conv::*;
pub use data::*;
pub use serde::*;
pub use log::*;
