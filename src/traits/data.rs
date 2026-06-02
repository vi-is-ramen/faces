/// Represents a set of flags that support common bitwise operations.
///
/// This trait serves as a marker for types that can be combined, intersected,
/// and negated using bitwise logic. It is typically implemented for enumerations
/// or newtype wrappers that represent flag values (e.g., file permissions,
/// configuration options, or state bits).
///
/// # Requirements
/// - `BitOr` – allows combining flags (union)
/// - `BitAnd` – allows intersecting flags (intersection)
/// - `Not` – allows complement/inversion of flags
/// - `Sized` – ensures the type has a known size at compile time
///
/// # Examples
/// ```
/// use faces::traits::{AbsFlags, Convertable, to};
/// use core::ops::{BitOr, BitAnd, Not};
///
/// #[derive(Clone, Copy, PartialEq, Eq)]
/// #[repr(u8)]
/// enum MyFlags {
///     A, B, C
/// };
/// 
/// impl Convertable<u8> for MyFlags {
///     fn to(self) -> u8 {
///         *&self as u8
///     }
/// }
/// 
/// impl Convertable<MyFlags> for u8 {
///     fn to(self) -> MyFlags {
///         match self {
///             val if val == MyFlags::A.to() => MyFlags::A,
///             val if val == MyFlags::B.to() => MyFlags::B,
///             val if val == MyFlags::C.to() => MyFlags::C,
///             _ => panic!("Invalid value"),
///         }
///     }
/// }
///
/// impl BitOr for MyFlags {
///     type Output = Self;
///     fn bitor(self, rhs: Self) -> Self { (to::<u8, _>(self) | to::<u8, _>(rhs)).to() }
/// }
///
/// impl BitAnd for MyFlags {
///     type Output = Self;
///     fn bitand(self, rhs: Self) -> Self { (to::<u8, _>(self) & to::<u8, _>(rhs)).to() }
/// }
///
/// impl Not for MyFlags {
///     type Output = Self;
///     fn not(self) -> Self { (!to::<u8, _>(self)).to() }
/// }
///
/// impl AbsFlags for MyFlags {}
/// ```
pub trait AbsFlags: core::ops::BitOr + core::ops::BitAnd + core::ops::Not
where
    Self: Sized,
{
}
