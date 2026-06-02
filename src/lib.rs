//! # Faces
//!
//! A collection of primitive types and ready‑to‑use traits for unifying interfaces
//! between different crates and software components. It helps you write adapters
//! between APIs that would otherwise be incompatible.
//!
//! ## Organisation
//!
//! The crate is split into two top‑level modules, each intended to grow over time
//! with many submodules:
//!
//! - [`types`] – Fundamental data types (addresses, indices, vectors, etc.).
//! - [`traits`] – Interface traits for conversion, flags, resource management,
//!   serialisation, and more.
//!
//! ## Extensibility
//!
//! The current number of submodules is deliberately small, but both `types` and
//! `traits` are designed to be extended infinitely. New submodules can be added
//! without breaking existing code, and the crate welcomes contributions.
//!
//! ## Usage
//!
//! Use the existing traits and types as building blocks for your own abstractions.
//! Because they follow common patterns, they make it easy to connect crates that
//! were not originally designed to work together.

#![allow(unused_imports)]

#![cfg(not(feature = "std"))]
#![no_std]

pub mod traits;
pub mod types;
