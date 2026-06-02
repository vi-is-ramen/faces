use crate::types::PageFrameNumber as PFN;

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
pub trait AbsPageFrameManager<F: crate::traits::AbsFlags> {
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

    // ----- Counters -----
    // Four independent counters per frame, typically used for reference counting,
    // page cache usage, or other allocation tracking.

    /// Increments counter 0 for the given page frame.
    fn inc0(&self, pfn: PFN);
    /// Increments counter 1 for the given page frame.
    fn inc1(&self, pfn: PFN);
    /// Increments counter 2 for the given page frame.
    fn inc2(&self, pfn: PFN);
    /// Increments counter 3 for the given page frame.
    fn inc3(&self, pfn: PFN);

    /// Decrements counter 0 for the given page frame.
    fn dec0(&self, pfn: PFN);
    /// Decrements counter 1 for the given page frame.
    fn dec1(&self, pfn: PFN);
    /// Decrements counter 2 for the given page frame.
    fn dec2(&self, pfn: PFN);
    /// Decrements counter 3 for the given page frame.
    fn dec3(&self, pfn: PFN);

    /// Returns the current value of counter 0 for the given page frame.
    fn get0(&self, pfn: PFN);
    /// Returns the current value of counter 1 for the given page frame.
    fn get1(&self, pfn: PFN);
    /// Returns the current value of counter 2 for the given page frame.
    fn get2(&self, pfn: PFN);
    /// Returns the current value of counter 3 for the given page frame.
    fn get3(&self, pfn: PFN);

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
}
