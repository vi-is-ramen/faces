// === PageFrameNumber === //

/// A page frame number (PFN) representing the index of a physical page frame.
///
/// Each page frame is typically 4 KiB (common page size), so the physical address
/// can be derived by shifting left by 12 bits: `PFN << 12 = PhysicalAddress`.
/// This type is a newtype wrapper around `usize` and provides conversions to/from
/// [`PhysicalAddress`] and raw `usize`.
///
/// # Examples
/// ```
/// use faces::{types::{PhysicalAddress, PageFrameNumber}, traits::to};
/// let pfn: PageFrameNumber = to(42);
/// let addr: PhysicalAddress = pfn.into();
/// assert_eq!(to::<usize, _>(addr), 42 << 12);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct PageFrameNumber(usize);

/// Converts a page frame number into its corresponding physical address.
///
/// The conversion assumes a page size of 4 KiB (4096 bytes), shifting the PFN left by 12 bits.
/// This is the inverse of `PhysicalAddress::into::<PageFrameNumber>`.
impl Into<PhysicalAddress> for PageFrameNumber {
    fn into(self) -> PhysicalAddress {
        PhysicalAddress(self.0 << 12)
    }
}

/// Converts a `PageFrameNumber` into the underlying `usize` value.
impl crate::traits::Convertable<usize> for PageFrameNumber {
    fn to(self) -> usize {
        self.0
    }
}

/// Converts a raw `usize` into a `PageFrameNumber`.
///
/// # Note
/// No validation is performed; the caller must ensure the value represents a valid PFN
/// within the expected range.
impl crate::traits::Convertable<PageFrameNumber> for usize {
    fn to(self) -> PageFrameNumber {
        PageFrameNumber(self)
    }
}

impl core::fmt::Display for PageFrameNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl serde::Serialize for PageFrameNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize as u64 to be portable across 32/64-bit targets.
        serializer.serialize_u64(self.0 as u64)
    }
}

impl<'de> serde::Deserialize<'de> for PageFrameNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Ok(PageFrameNumber(value as usize))
    }
}

// === PhysicalAddress === //

/// A physical memory address.
///
/// Represents a raw address in physical memory space. This type is a newtype wrapper
/// around `usize` and provides conversions to/from [`PageFrameNumber`] and raw `usize`.
/// The conversion to `PageFrameNumber` assumes a page size of 4 KiB, shifting right by 12 bits.
///
/// # Examples
/// ```
/// use faces::{types::{PhysicalAddress, PageFrameNumber}, traits::{to, Convertable}};
/// let addr: PhysicalAddress = to(0x1000usize);
/// let pfn: PageFrameNumber = addr.into();
/// assert_eq!(to::<usize, _>(pfn), 1);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct PhysicalAddress(usize);

/// Converts a physical address into the page frame number that contains it.
///
/// This discards the page offset (lowest 12 bits), yielding the PFN of the page
/// in which the address resides. Equivalent to `address >> 12`.
impl Into<PageFrameNumber> for PhysicalAddress {
    fn into(self) -> PageFrameNumber {
        PageFrameNumber(self.0 >> 12)
    }
}

/// Converts a `PhysicalAddress` into the underlying `usize` value.
impl crate::traits::Convertable<usize> for PhysicalAddress {
    fn to(self) -> usize {
        self.0
    }
}

/// Converts a raw `usize` into a `PhysicalAddress`.
///
/// No validation is performed; the caller must ensure the address is a valid
/// physical address for the target system.
impl crate::traits::Convertable<PhysicalAddress> for usize {
    fn to(self) -> PhysicalAddress {
        PhysicalAddress(self)
    }
}

impl core::fmt::Display for PhysicalAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl serde::Serialize for PhysicalAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.0 as u64)
    }
}

impl<'de> serde::Deserialize<'de> for PhysicalAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Ok(PhysicalAddress(value as usize))
    }
}

// === VirtualAddress === //

/// A virtual memory address.
///
/// Represents an address in the virtual address space of a process. This type is
/// a newtype wrapper around `usize` and provides conversions to/from raw `usize`.
/// Unlike physical addresses, the interpretation of a virtual address is
/// context-dependent (e.g., page tables determine the mapping to physical frames).
///
/// # Examples
/// ```
/// use faces::{types::{VirtualAddress, PageFrameNumber}, traits::{to, Convertable}};
/// let va: VirtualAddress = to(0x7fff_0000);
/// let raw: usize = va.to();
/// assert_eq!(raw, 0x7fff_0000);
/// let restored: VirtualAddress = to(raw);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct VirtualAddress(usize);

/// Converts a virtual address into the underlying `usize` value.
impl crate::traits::Convertable<usize> for VirtualAddress {
    fn to(self) -> usize {
        self.0
    }
}

/// Converts a raw `usize` into a `VirtualAddress`.
///
/// No validation is performed; the caller must ensure the raw value represents
/// a valid virtual address in the current context.
impl crate::traits::Convertable<VirtualAddress> for usize {
    fn to(self) -> VirtualAddress {
        VirtualAddress(self)
    }
}

impl core::fmt::Display for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl serde::Serialize for VirtualAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.0 as u64)
    }
}

impl<'de> serde::Deserialize<'de> for VirtualAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Ok(VirtualAddress(value as usize))
    }
}

/// Converts a virtual address into a shared reference of type `&'a T`.
///
/// This implementation of [`UnsafeConvertable`] allows a `VirtualAddress` to be
/// interpreted as a reference to a value of type `T` located at that address
/// in the virtual address space.
///
/// # Lifetimes
/// * `'a` – The lifetime of the resulting reference. The caller must ensure
///   that the memory at the virtual address remains valid and accessible for
///   the entire duration `'a`.
///
/// # Type Parameter `T`
/// The type `T` of the referenced value. No compile‑time checks guarantee that
/// the memory actually contains a valid `T`; this is entirely a safety
/// precondition for the caller.
///
/// # Safety
/// The caller of this unsafe method must guarantee **all** of the following:
/// * The virtual address (`self.0`) is correctly aligned for type `T`.
/// * The memory at that address contains a valid value of type `T` (e.g., it
///   has been properly initialized and is not a dangling pointer).
/// * The memory remains valid and immutable for at least the lifetime `'a`
///   (i.e., no mutable accesses occur concurrently or afterwards, and the
///   memory is not deallocated or reused).
/// * The virtual address is mapped and readable in the current address space.
/// * No aliasing rules of Rust are violated (e.g., the same memory is not
///   simultaneously borrowed mutably elsewhere).
///
/// Failure to uphold any of these conditions leads to undefined behavior.
unsafe impl<'a, T> crate::traits::UnsafeConvertable<&'a T> for VirtualAddress {
    unsafe fn to(self) -> &'a T {
        unsafe {
            (self.0 as *const T).as_ref_unchecked()
        }
    }
}

/// Converts a virtual address into a mutable reference of type `&'a mut T`.
///
/// This implementation of [`UnsafeConvertable`] allows a `VirtualAddress` to be
/// interpreted as a mutable reference to a value of type `T` located at that
/// address in the virtual address space. Because the reference is `mut`, the
/// caller can modify the pointed-to memory, subject to Rust's aliasing rules.
///
/// # Lifetimes
/// * `'a` – The lifetime of the resulting mutable reference. The caller must
///   guarantee that the memory at the virtual address remains valid and
///   exclusively accessible for the entire duration `'a`.
///
/// # Type Parameter `T`
/// The type `T` of the referenced value. No compile‑time checks ensure that
/// the memory actually contains a valid `T`; this is a safety precondition
/// for the caller.
///
/// # Safety
/// The caller of this unsafe method must guarantee **all** of the following:
/// * The virtual address (`self.0`) is correctly aligned for type `T`.
/// * The memory at that address contains a valid value of type `T` (properly
///   initialized, not a dangling pointer).
/// * The memory remains valid and **exclusively** accessible for the lifetime
///   `'a` – no other references (shared or mutable) to the same memory may
///   exist concurrently.
/// * The virtual address is mapped and writable in the current address space.
/// * The memory is not deallocated or reused during `'a`.
/// * No data races occur – the caller must have appropriate synchronization
///   if the address is shared across threads.
///
/// Failure to uphold any of these conditions leads to undefined behavior.
///
/// # Examples
/// ```
/// use faces::traits::{UnsafeConvertable, to};
/// use faces::types::VirtualAddress;
///
/// let mut value = 42u32;
/// let addr: VirtualAddress = to(&mut value as *mut u32 as usize);
/// unsafe {
///     let ref_mut: &mut u32 = addr.to();
///     *ref_mut = 100;
/// }
/// assert_eq!(value, 100);
/// ```
unsafe impl<'a, T> crate::traits::UnsafeConvertable<&'a mut T> for VirtualAddress {
    unsafe fn to(self) -> &'a mut T {
        unsafe {
            (self.0 as *mut T).as_mut_unchecked()
        }
    }
}
