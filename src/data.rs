//! Basic data type traits.

use core::marker::Sized;

/// A marker trait for types that behave as a set of flags.
///
/// Types implementing `AbsFlags` must support bitwise operations `BitOr`, `BitAnd`,
/// and `Not`. This trait is typically used as a bound for flag types in other
/// abstractions (e.g., page frame managers).
pub trait AbsFlags: core::ops::BitOr + core::ops::BitAnd + core::ops::Not where Self: Sized {}
