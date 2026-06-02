//! # Logging trait
//!
//! This module re‑exports the core logging trait from the `log` crate:
//! [`Log`].
//!
//! The re‑export is provided for convenience, so that users of `faces` can
//! implement or use the `Log` trait without adding an explicit dependency on
//! the `log` crate themselves. This is especially useful in `no_std`
//! environments where you want a common logging interface.
//!
//! # Examples
//!
//! ```
//! use faces::traits::Log;
//!
//! struct MyLogger;
//!
//! impl Log for MyLogger {
//!     fn enabled(&self, metadata: &log::Metadata) -> bool {
//!         true
//!     }
//!
//!     fn log(&self, record: &log::Record) {
//!         // custom logging logic
//!     }
//!
//!     fn flush(&self) {}
//! }
//! ```
//!
//! For more details about logging, refer to the [`log` crate documentation](https://docs.rs/log).

pub use ::log::Log;
