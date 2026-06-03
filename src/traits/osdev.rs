use crate::types::{PageFrameNumber as PFN, PhysicalAddress, VirtualAddress};

/// Abstract interface for managing page frames with associated flags and counters.
///
/// This trait defines the core operations for a page frame manager (PFM) that controls
/// a range of physical or virtual page frames. Each frame is identified by a
/// [`PageFrameNumber`] (PFN). The manager supports:
/// - Flag operations (set, clear, check) using a generic flags type `F` that implements
///   [`AbsFlags`].
/// - Synchronization primitives (lock, free) for mutual exclusion on individual frames.
/// - Boundary queries (min/max PFN) to determine the managed range.
/// - Four independent counters per frame (inc/dec/get) for reference counting or other uses.
/// - Presence check to test if a PFN belongs to this manager.
/// - Raw pointer access (unsafe) to the memory associated with a frame.
///
/// # Type Parameters
/// * `F` – A flags type that implements [`AbsFlags`], used for per-frame flag operations.
///
/// # Notes
/// Implementations must ensure that `lock()` and `free()` provide appropriate
/// synchronization semantics (e.g., acquiring/releasing a spinlock, mutex, or similar).
/// Counter operations (`inc*`, `dec*`, `get*`) are typically expected to be atomic.
pub trait AbsPageFrameManager<F: crate::traits::AbsFlags, T> {
    // ----- Flags -----

    /// Sets the specified flags on the given page frame.
    ///
    /// This operation performs a bitwise OR of the current flags with `flag`.
    /// The exact concurrency behavior (e.g., atomicity) is implementation-defined,
    /// but should be safe for concurrent calls.
    ///
    /// # Arguments
    /// * `pfn` – The page frame number to modify.
    /// * `flag` – Flags to set.
    ///
    /// # Panics
    /// May panic if `pfn` is not managed by this manager (see [`present`](Self::present)).
    fn set_flags(&self, pfn: PFN, flag: F);

    /// Clears the specified flags on the given page frame.
    ///
    /// This operation performs a bitwise AND of the current flags with the complement of `flag`.
    /// The exact concurrency behavior is implementation-defined.
    ///
    /// # Arguments
    /// * `pfn` – The page frame number to modify.
    /// * `flag` – Flags to clear.
    ///
    /// # Panics
    /// May panic if `pfn` is not managed by this manager.
    fn clear_flags(&self, pfn: PFN, flag: F);

    /// Checks whether the specified flags are set on the given page frame.
    ///
    /// Returns `true` if all bits in `flag` are set in the current flags,
    /// `false` otherwise.
    ///
    /// # Arguments
    /// * `pfn` – The page frame number to query.
    /// * `flag` – Flags to test.
    ///
    /// # Panics
    /// May panic if `pfn` is not managed by this manager.
    fn check_flags(&self, pfn: PFN, flag: F) -> bool;

    // ----- Synchronization -----

    /// Locks the given page frame for exclusive access.
    ///
    /// The exact locking semantics (spinlock, mutex, recursive, etc.) are implementation-defined.
    /// Typically, this must be called before performing a series of operations that require
    /// consistency. A matching call to [`free`](Self::free) should release the lock.
    ///
    /// # Arguments
    /// * `pfn` – The page frame to lock.
    ///
    /// # Panics
    /// May panic if `pfn` is not managed, or if the lock is already held by the same context
    /// and recursive locks are not supported.
    fn lock(&self, pfn: PFN);

    /// Unlocks (frees) the given page frame, releasing a previously acquired lock.
    ///
    /// This must be called after a successful [`lock`](Self::lock). The behavior is undefined
    /// if called on an unlocked frame or from a different context than the lock owner.
    ///
    /// # Arguments
    /// * `pfn` – The page frame to unlock.
    ///
    /// # Panics
    /// May panic if `pfn` is not managed or if the lock is not currently held.
    fn free(&self, pfn: PFN);

    // ----- Boundary -----

    /// Returns the smallest (starting) page frame number managed by this manager.
    ///
    /// All valid PFNs for this manager satisfy `self.min() <= pfn <= self.max()`.
    fn min(&self) -> PFN;

    /// Returns the largest (ending) page frame number managed by this manager.
    ///
    /// All valid PFNs for this manager satisfy `self.min() <= pfn <= self.max()`.
    fn max(&self) -> PFN;

    // ----- Presence -----

    /// Checks whether the given page frame is managed by this manager.
    ///
    /// Returns `true` if `pfn` is within the bounds `[min(), max()]` and otherwise
    /// considered present (e.g., not a hole, not reserved). Implementations may
    /// have additional restrictions beyond the numeric range.
    fn present(&self, pfn: PFN) -> bool;

    // ----- Raw access -----

    /// Returns a raw constant pointer to the memory associated with the given page frame.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `pfn` is managed by this manager and is valid (e.g., not freed or unmapped).
    /// - No mutable aliasing occurs; the returned pointer may be shared.
    /// - The memory is not accessed after the frame is freed or reused.
    ///
    /// # Arguments
    /// * `pfn` – The page frame number.
    ///
    /// # Returns
    /// A `*const ()` pointing to the start of the frame's memory.
    unsafe fn get_ptr(&self, pfn: PFN) -> *const ();

    /// Returns a raw mutable pointer to the memory associated with the given page frame.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `pfn` is managed by this manager and is valid.
    /// - No other references (mutable or immutable) exist to the same memory.
    /// - Proper synchronization (e.g., through `lock`/`free`) is used to avoid data races.
    /// - The memory is not accessed after the frame is freed or reused.
    ///
    /// # Arguments
    /// * `pfn` – The page frame number.
    ///
    /// # Returns
    /// A `*mut ()` pointing to the start of the frame's memory.
    unsafe fn get_mut(&self, pfn: PFN) -> *mut ();

    // ----- Field access (with automatic locking) -----

    /// Returns a shared reference to a specific field inside the page frame.
    ///
    /// This method locks the page frame for the entire duration of the returned
    /// reference's lifetime `'a`. The lock is acquired automatically and released
    /// only after the reference goes out of scope.
    ///
    /// # Type Parameters
    /// * `U` – The type of the field to access. The implementation must ensure
    ///   that the field selector `fid` correctly identifies a field of this type.
    ///
    /// # Arguments
    /// * `pfn` – The page frame number.
    /// * `fid` – A field selector (typically an enum) identifying which field to access.
    ///
    /// # Returns
    /// A shared reference to the field, valid for the lifetime `'a`.
    ///
    /// # Safety
    /// This method is `unsafe` because the implementation must guarantee:
    /// - The field selector `fid` is valid for the given `pfn`.
    /// - The returned reference does not outlive the frame’s lock.
    /// - No mutable aliasing occurs (callers must not simultaneously call
    ///   `field_mut` on the same frame while this reference exists).
    /// - The memory layout of the field matches `U`.
    ///
    /// Implementations should document any additional safety conditions.
    unsafe fn field<'a, U>(&self, pfn: PFN, fid: T) -> &'a U;

    /// Returns a mutable reference to a specific field inside the page frame.
    ///
    /// This method locks the page frame for the entire duration of the returned
    /// reference's lifetime `'a`. The lock is acquired automatically and released
    /// only after the reference goes out of scope.
    ///
    /// # Type Parameters
    /// * `U` – The type of the field to access. The implementation must ensure
    ///   that the field selector `fid` correctly identifies a field of this type.
    ///
    /// # Arguments
    /// * `pfn` – The page frame number.
    /// * `fid` – A field selector (typically an enum) identifying which field to access.
    ///
    /// # Returns
    /// A mutable reference to the field, valid for the lifetime `'a`.
    ///
    /// # Safety
    /// This method is `unsafe` because the implementation must guarantee:
    /// - The field selector `fid` is valid for the given `pfn`.
    /// - The returned reference does not outlive the frame’s lock.
    /// - No other references (shared or mutable) to the same field exist concurrently.
    /// - The memory layout of the field matches `U`.
    ///
    /// Implementations should document any additional safety conditions.
    unsafe fn field_mut<'a, U>(&self, pfn: PFN, fid: T) -> &'a mut U;
}

/// Abstract interface for translating virtual addresses to physical addresses and vice versa.
///
/// This trait provides a bidirectional conversion between [`VirtualAddress`] and
/// [`PhysicalAddress`]. It is intended for systems with a linear, deterministic
/// mapping between the two address spaces – for example, when using identity mapping
/// (virtual == physical) or a fixed offset.
///
/// # Correctness
///
/// **This trait is inherently unsafe to implement and use** in most real systems,
/// because address translation depends on the current page tables, memory layout,
/// and CPU state. Implementations must document any preconditions (e.g., that the
/// address is mapped, that the translation is globally valid, or that the caller
/// holds the appropriate locks).
///
/// Calling `as_phys` or `as_virt` may produce a meaningless or invalid address
/// if the translation is not valid for the given input. It is the caller’s
/// responsibility to ensure the address is translatable.
///
/// # Examples
///
/// ```
/// # use faces::types::{VirtualAddress, PhysicalAddress};
/// # use faces::traits::AbsAddressTranslator;
/// # use faces::traits::to;
/// struct IdentityTranslator;
///
/// impl AbsAddressTranslator for IdentityTranslator {
///     fn as_phys(v: VirtualAddress) -> PhysicalAddress {
///         // Identity mapping: virtual == physical
///         to(to(v))
///     }
///     fn as_virt(p: PhysicalAddress) -> VirtualAddress {
///         to(to(p))
///     }
/// }
///
/// let virt = to(0x1000u64 as usize);
/// let phys = IdentityTranslator::as_phys(virt);
/// assert_eq!(to::<usize, _>(virt), to::<usize, _>(phys));
/// ```
///
/// # Platform‑specific notes
///
/// - On many kernels, a single global translation function exists for the
///   current address space. Implementations may store a `&'static` reference
///   to the active page table.
/// - For systems with multiple address spaces (e.g., processes), the trait
///   should be implemented on a context‑carrying type (like a `PageTable` or
///   `AddressSpace`), rather than a zero‑sized type.
/// - This trait does not provide any error handling – invalid addresses lead
///   to unspecified results (e.g., garbage or a panic). Use fallible wrappers
///   when needed.
pub trait AbsAddressTranslator {
    /// Converts a virtual address to the corresponding physical address.
    ///
    /// # Arguments
    /// * `v` – A virtual address to translate.
    ///
    /// # Returns
    /// The physical address that `v` maps to, according to the translation
    /// rules defined by the implementor.
    ///
    /// # Safety / Correctness
    /// Calling this method may be **unsound** if `v` is not a valid mapped
    /// virtual address in the current translation context. Implementations
    /// may panic, return an unmapped physical address, or produce a value
    /// that leads to undefined behavior when used as a physical address.
    ///
    /// The caller must ensure that the translation is meaningful (e.g., the
    /// virtual address is part of a known mapping, and the page tables are
    /// not being concurrently modified).
    fn as_phys(v: VirtualAddress) -> PhysicalAddress;

    /// Converts a physical address to a virtual address.
    ///
    /// This is the inverse of [`as_phys`](Self::as_phys). Not all physical
    /// addresses are mapped into the virtual address space; calling this
    /// function may produce an invalid virtual address.
    ///
    /// # Arguments
    /// * `p` – A physical address to translate.
    ///
    /// # Returns
    /// A virtual address that (presumably) maps to `p`, according to the
    /// translation rules defined by the implementor.
    ///
    /// # Safety / Correctness
    /// The same safety considerations as [`as_phys`](Self::as_phys) apply.
    /// The caller must know that `p` is actually mapped at the returned
    /// virtual address and that the mapping is valid for the current context.
    fn as_virt(p: PhysicalAddress) -> VirtualAddress;
}
