//! # Types
//!
//! This module holds fundamental data types that are useful across many domains.
//! It is organised into submodules, each focusing on a specific category of types.
//!
//! ## Current submodules
//!
//! - `osdev` – Types for memory addressing and page indices (useful beyond OS development).
//!
//! ## Future expansion
//!
//! Many more submodules are planned (e.g. 2D/3D vectors, fixed‑point numbers,
//! strongly‑typed IDs, bit masks, etc.). The crate is designed to keep growing,
//! and contributions are very welcome.
//!
//! All types within these submodules strive to be:
//! - Lightweight (newtype wrappers where appropriate).
//! - Interoperable (via the [`Convertable`] family of traits).
//! - Safe by default, with unsafe operations clearly marked.

pub mod osdev;

pub use osdev::*;
