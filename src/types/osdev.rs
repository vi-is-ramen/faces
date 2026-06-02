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
