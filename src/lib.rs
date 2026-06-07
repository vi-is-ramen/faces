#![cfg_attr(not(feature = "std"), no_std)]

//! # Faces
//!
//! A crate that provides primitive types and a collection of ready‑to‑use traits
//! to unify interfaces between different crates and software components.
//!
//! The name reflects the idea of a “face” – a common interface that different
//! systems can present to each other.

pub mod conv;
pub mod data;
pub mod mm;
#[cfg(feature = "log")]
pub mod log;
#[cfg(feature = "serde")]
pub mod serde;
pub mod sync;

pub use conv::*;
pub use data::*;
pub use mm::*;
#[cfg(feature = "log")]
pub use log::*;
#[cfg(feature = "serde")]
pub use serde::*;
pub use sync::*;
