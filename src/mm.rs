//! Memory management primitives: page frame numbers, physical/virtual addresses,
//! and abstract interfaces for page frame managers and address translation.

use crate::*;

use core::{
    convert::From,
    marker::{Copy, Sync},
    clone::Clone,
    fmt::Debug,
    derive,
    write,
};

#[doc(hidden)]
type PFN = PageFrameNumber;

/// An abstract page frame manager.
///
/// This trait provides operations to manage page frames (typically 4 KiB blocks)
/// and associate platform‑specific flags with them.
pub trait AbsPageFrameManager {
    /// Type representing flags that can be set on a page frame.
    /// Must implement [`AbsFlags`].
    type Flags: AbsFlags;

    /// Type used to access a page frame (e.g., a reference or a descriptor).
    type Access: Sync + Send;

    /// Sets the given flags on the specified page frame.
    fn set_flags(&self, pfn: PFN, flag: Self::Flags);

    /// Clears the given flags from the specified page frame.
    fn clear_flags(&self, pfn: PFN, flag: Self::Flags);

    /// Checks whether the given flags are set on the specified page frame.
    fn check_flags(&self, pfn: PFN, flag: Self::Flags) -> bool;

    /// Returns the smallest valid page frame number.
    fn min(&self) -> PFN;

    /// Returns the largest valid page frame number.
    fn max(&self) -> PFN;

    /// Returns `true` if the page frame is present (allocated/mapped).
    fn present(&self, pfn: PFN) -> bool;

    /// Returns the accessor object for the given page frame.
    fn get(&self, pfn: PFN) -> Self::Access;
}

/// An abstract address translator between virtual and physical addresses.
pub trait AbsAddressTranslator {
    /// Converts a virtual address to a physical address.
    fn as_phys(v: VirtualAddress) -> PhysicalAddress;

    /// Converts a physical address to a virtual address.
    fn as_virt(p: PhysicalAddress) -> VirtualAddress;
}

/// A page frame number – index of a physical memory page (usually 4 KiB).
#[derive(Clone, Copy, Debug)]
pub struct PageFrameNumber(usize);

impl PageFrameNumber {
    /// Creates a new page frame number from a raw `usize`.
    pub const fn new(n: usize) -> Self {
        Self(n)
    }
}

impl Default for PageFrameNumber {
    fn default() -> Self {
        Self(0)
    }
}

impl From<PageFrameNumber> for PhysicalAddress {
    fn from(value: PageFrameNumber) -> Self {
        PhysicalAddress(to::<usize, _>(value) << 12)
    }
}

impl Convertable<usize> for PageFrameNumber {
    fn to(self) -> usize {
        self.0
    }
}

impl Convertable<PageFrameNumber> for usize {
    fn to(self) -> PageFrameNumber {
        PageFrameNumber(self)
    }
}

impl core::fmt::Display for PageFrameNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for PageFrameNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_u64(self.0 as u64)
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for PageFrameNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Ok(PageFrameNumber(value as usize))
    }
}

impl core::ops::Add for PageFrameNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        to(self.0 + rhs.0)
    }
}

impl core::ops::AddAssign for PageFrameNumber {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl core::ops::BitAnd for PageFrameNumber {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        to(self.0 & rhs.0)
    }
}

impl core::ops::BitAndAssign for PageFrameNumber {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl core::ops::BitOr for PageFrameNumber {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        to(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PageFrameNumber {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl core::ops::BitXor for PageFrameNumber {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        to(self.0 ^ rhs.0)
    }
}

impl core::ops::BitXorAssign for PageFrameNumber {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl core::ops::Div for PageFrameNumber {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        to(self.0 / rhs.0)
    }
}

impl core::ops::DivAssign for PageFrameNumber {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl core::ops::Mul for PageFrameNumber {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        to(self.0 * rhs.0)
    }
}

impl core::ops::MulAssign for PageFrameNumber {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl core::ops::Neg for PageFrameNumber {
    type Output = Self;
    fn neg(self) -> Self::Output {
        to(!self.0 - 1)
    }
}

impl core::ops::Not for PageFrameNumber {
    type Output = Self;
    fn not(self) -> Self::Output {
        to(!self.0)
    }
}

/// A physical memory address (usually an offset into the physical address space).
#[derive(Clone, Copy, Debug)]
pub struct PhysicalAddress(usize);

impl From<PhysicalAddress> for PageFrameNumber {
    fn from(value: PhysicalAddress) -> PageFrameNumber {
        PageFrameNumber(to::<usize, _>(value) >> 12)
    }
}

impl PhysicalAddress {
    /// Creates a new page frame number from a raw `usize`.
    pub const fn new(n: usize) -> Self {
        Self(n)
    }
}

impl Default for PhysicalAddress {
    fn default() -> Self {
        Self(0)
    }
}

impl Convertable<usize> for PhysicalAddress {
    fn to(self) -> usize {
        self.0
    }
}

impl Convertable<PhysicalAddress> for usize {
    fn to(self) -> PhysicalAddress {
        PhysicalAddress(self)
    }
}

impl core::fmt::Display for PhysicalAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for PhysicalAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_u64(self.0 as u64)
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for PhysicalAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Ok(PhysicalAddress(value as usize))
    }
}

/// A virtual memory address (an address in the virtual address space of a process).
#[derive(Clone, Copy, Debug)]
pub struct VirtualAddress(usize);

impl VirtualAddress {
    /// Creates a new page frame number from a raw `usize`.
    pub const fn new(n: usize) -> Self {
        Self(n)
    }
}

impl Default for VirtualAddress {
    fn default() -> Self {
        Self(0)
    }
}

impl Convertable<usize> for VirtualAddress {
    fn to(self) -> usize {
        self.0
    }
}

impl Convertable<VirtualAddress> for usize {
    fn to(self) -> VirtualAddress {
        VirtualAddress(self)
    }
}

impl core::fmt::Display for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for VirtualAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_u64(self.0 as u64)
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for VirtualAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Ok(VirtualAddress(value as usize))
    }
}

/// Unsafe conversion from a virtual address to a shared reference.
///
/// ## Safety
/// The caller must ensure that the virtual address points to a valid, properly
/// initialised value of type `T` that lives for at least the lifetime `'a`.
unsafe impl<'a, T> UnsafeConvertable<&'a T> for VirtualAddress {
    unsafe fn unsafe_to(self) -> &'a T {
        unsafe { (self.0 as *const T).as_ref_unchecked() }
    }
}

/// Unsafe conversion from a virtual address to a mutable reference.
///
/// ## Safety
/// The caller must ensure that the virtual address points to a valid, properly
/// initialised value of type `T` that lives for at least the lifetime `'a`,
/// and that no other references (shared or mutable) to the same memory exist.
unsafe impl<'a, T> UnsafeConvertable<&'a mut T> for VirtualAddress {
    unsafe fn unsafe_to(self) -> &'a mut T {
        unsafe { (self.0 as *mut T).as_mut_unchecked() }
    }
}

/// An abstract page table mapper.
///
/// This trait provides core operations to manage virtual‑to‑physical address mappings
/// in a platform‑agnostic way. It allows protecting (mapping) a range of virtual
/// addresses with platform‑specific flags, removing protections (unmapping), and
/// querying existing page table entries.
pub trait AbsMapper<F: AbsFlags, PageTable> {
    /// The type of errors that can occur during mapping or query operations.
    /// Must implement [`Debug`].
    type Error: Debug;

    /// Creates a new instance of the mapper.
    fn new() -> Self;

    /// Maps a range of virtual addresses to physical frames with the given flags.
    ///
    /// For each page in the range `[v, v + count * PAGE_SIZE)`, the virtual page
    /// is mapped to the physical frame starting at `p + i * PAGE_SIZE` (where `i`
    /// is the page index) with the specified `flags`.
    ///
    /// # Parameters
    /// - `v`: Starting virtual address.
    /// - `p`: Starting physical address.
    /// - `count`: Number of consecutive pages to map.
    /// - `flags`: Platform‑specific flags to apply to the mapping.
    ///
    /// # Errors
    /// Returns an error if the address range is invalid, the mapping cannot be
    /// performed (e.g., due to insufficient privileges), or if the underlying
    /// platform rejects the operation.
    fn protect(&mut self, v: VirtualAddress, p: PhysicalAddress, count: usize, flags: F) -> Result<(), Self::Error>;

    /// Unmaps a range of virtual addresses.
    ///
    /// Removes any existing mapping for each page in the range
    /// `[v, v + count * PAGE_SIZE)`. After this operation, accessing those
    /// virtual addresses will typically cause a page fault.
    ///
    /// # Parameters
    /// - `v`: Starting virtual address.
    /// - `count`: Number of consecutive pages to unmap.
    ///
    /// # Errors
    /// Returns an error if the address range is invalid or if the underlying
    /// platform fails to remove the mappings.
    fn unprotect(&mut self, v: VirtualAddress, count: usize) -> Result<(), Self::Error>;

    /// Queries the page table entry for a given virtual address.
    ///
    /// Returns platform‑specific page table information (e.g., physical frame
    /// number, access flags, presence status) associated with the mapping at `v`.
    ///
    /// # Parameters
    /// - `v`: Virtual address to query.
    ///
    /// # Errors
    /// Returns an error if the address is not mapped, the query fails, or if the
    /// underlying page table cannot be accessed.
    fn query(&self, v: VirtualAddress) -> Result<PageTable, Self::Error>;
}

/// An abstract page table mapper that accepts additional hints for performance or
/// platform‑specific optimisations.
///
/// This trait extends [`AbsMapper`] with methods that allow passing a `hints`
/// parameter. Hints can be used to convey information such as expected access
/// patterns, cache policies, or other platform‑dependent guidance. The default
/// hint type is `()`, meaning no hints are required.
pub trait AbsHintedMapper<F: AbsFlags, PageTable, Hints = ()>: AbsMapper<F, PageTable> {
    /// Maps a range of virtual addresses with the given flags and hints.
    ///
    /// Behaves identically to [`AbsMapper::protect`] but also accepts a `hints`
    /// parameter that may influence how the mapping is established (e.g., prefetching,
    /// large page hints, or NUMA node preferences).
    ///
    /// # Parameters
    /// - `v`: Starting virtual address.
    /// - `p`: Starting physical address.
    /// - `count`: Number of consecutive pages to map.
    /// - `flags`: Platform‑specific flags for the mapping.
    /// - `hints`: Additional guidance for the mapper.
    fn protect_hinted(&mut self, v: VirtualAddress, p: PhysicalAddress, count: usize, flags: F, hints: Hints) -> Result<(), Self::Error>;

    /// Unmaps a range of virtual addresses with the given hints.
    ///
    /// Behaves identically to [`AbsMapper::unprotect`] but accepts a `hints`
    /// parameter that may influence the unmapping operation (e.g., deferred TLB
    /// invalidation hints).
    ///
    /// # Parameters
    /// - `v`: Starting virtual address.
    /// - `count`: Number of consecutive pages to unmap.
    /// - `hints`: Additional guidance for the mapper.
    fn unprotect_hinted(&mut self, v: VirtualAddress, count: usize, hints: Hints) -> Result<(), Self::Error>;
}
