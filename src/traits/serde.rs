//! # Serde traits
//!
//! This module re‑exports the core serialisation traits from the `serde` crate:
//! [`Serialize`] and [`Deserialize`].
//!
//! The re‑export is provided for convenience, so that users of `faces` can access
//! these traits without adding an explicit dependency on `serde` themselves.
//! Many types defined in `faces` implement these traits, and you are encouraged
//! to do the same for your own types when appropriate.
//!
//! # Examples
//!
//! ```
//! use faces::traits::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyData {
//!     value: u32,
//! }
//! ```
//!
//! For more details about serialisation, refer to the [`serde` documentation](https://docs.rs/serde).

pub use ::serde::{Deserialize, Serialize};
