//! Abstract synchronization primitives.

use core::ops::{Deref, DerefMut};
use core::time::Duration;

/// Errors that can occur when trying to acquire a lock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockError {
    /// The lock could not be acquired because it would block
    /// (e.g., `try_lock` failed or a timeout expired).
    WouldBlock,
    /// The lock is poisoned (e.g., a previous holder panicked while holding the lock).
    Poisoned,
}

impl core::fmt::Display for LockError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LockError::WouldBlock => write!(f, "lock is already held or timed out"),
            LockError::Poisoned => write!(f, "lock is poisoned"),
        }
    }
}

/// A relaxation strategy used by lock implementations to avoid busy‑waiting.
pub trait AbsRelaxStrategy: Send + Sync + 'static {
    /// Called by a lock when it needs to wait or yield.
    fn relax();
}

/// An abstract mutual exclusion lock.
///
/// Implementations must provide a guard that dereferences to the protected data.
pub trait AbsMutex<T: ?Sized + Send, R: AbsRelaxStrategy>: Send + Sync {
    /// The guard type returned after successfully locking the mutex.
    /// It must implement `DerefMut<Target = T>` and be `Send`.
    type Guard<'a>: DerefMut<Target = T> + Send + 'a
    where
        Self: 'a;

    /// Acquires the mutex, blocking the current thread (or using the relax strategy)
    /// until it is available.
    ///
    /// Returns an error if the lock is poisoned.
    fn lock(&self) -> Result<Self::Guard<'_>, LockError>;

    /// Attempts to acquire the mutex without blocking.
    ///
    /// Returns `LockError::WouldBlock` if the mutex is already locked.
    fn try_lock(&self) -> Result<Self::Guard<'_>, LockError>;
}

/// An abstract read‑write lock.
pub trait AbsRwLock<T: ?Sized + Send + Sync, R: AbsRelaxStrategy>: Send + Sync {
    /// Guard for a read lock. Implements `Deref<Target = T>`.
    type ReadGuard<'a>: Deref<Target = T> + Send + 'a
    where
        Self: 'a;

    /// Guard for a write lock. Implements `DerefMut<Target = T>`.
    type WriteGuard<'a>: DerefMut<Target = T> + Send + 'a
    where
        Self: 'a;

    /// Acquires a read lock, blocking until it is available.
    fn read(&self) -> Result<Self::ReadGuard<'_>, LockError>;

    /// Attempts to acquire a read lock without blocking.
    fn try_read(&self) -> Result<Self::ReadGuard<'_>, LockError>;

    /// Acquires a write lock, blocking until it is available.
    fn write(&self) -> Result<Self::WriteGuard<'_>, LockError>;

    /// Attempts to acquire a write lock without blocking.
    fn try_write(&self) -> Result<Self::WriteGuard<'_>, LockError>;
}

/// An abstract one‑time initialisation primitive (like `Once` or `OnceLock`).
pub trait AbsOnce: Send + Sync + 'static {
    /// Creates a new `AbsOnce` instance in the uninitialised state.
    fn new() -> Self;

    /// Executes the given closure exactly once, no matter how many times this method is called.
    fn call_once<F>(&self, f: F)
    where
        F: FnOnce();

    /// Returns `true` if the initialisation routine has already completed.
    fn is_completed(&self) -> bool;

    /// Blocks the current thread until the initialisation routine has completed.
    fn wait(&self);
}

/// An abstract lazily initialised value (like `LazyLock`).
pub trait AbsLazy<T: ?Sized + Send + Sync>: Deref<Target = T> + Send + Sync + 'static {
    /// Forces initialisation of the lazy value, returning a reference to it.
    fn force(&self) -> &T;

    /// Consumes the lazy wrapper and returns the inner value if it has been initialised.
    ///
    /// # Panics
    /// May panic if the value has not been initialised yet.
    fn into_inner(self) -> T
    where
        Self: Sized;
}

/// Extension of [`AbsMutex`] with timeout capabilities.
pub trait AbsMutexTimeout<T: ?Sized + Send, R: AbsRelaxStrategy>: AbsMutex<T, R> {
    /// Tries to acquire the lock until the given closure returns `true` (timeout expired).
    ///
    /// The closure `is_expired` is called repeatedly to check for timeout.
    fn try_lock_until<F>(&self, is_expired: F) -> Result<Self::Guard<'_>, LockError>
    where
        F: Fn() -> bool;

    /// Tries to acquire the lock for at most `timeout` duration.
    ///
    /// The closure `now` returns the current time measurement (e.g., monotonic time).
    fn try_lock_for<F>(&self, timeout: Duration, now: F) -> Result<Self::Guard<'_>, LockError>
    where
        F: Fn() -> Duration,
    {
        let start = now();
        self.try_lock_until(|| now().saturating_sub(start) >= timeout)
    }
}

/// Extension of [`AbsRwLock`] with timeout capabilities.
pub trait AbsRwLockTimeout<T: ?Sized + Send + Sync, R: AbsRelaxStrategy>: AbsRwLock<T, R> {
    /// Tries to acquire a read lock until the timeout expires.
    fn try_read_until<F>(&self, is_expired: F) -> Result<Self::ReadGuard<'_>, LockError>
    where
        F: Fn() -> bool;

    /// Tries to acquire a read lock for at most `timeout` duration.
    fn try_read_for<F>(&self, timeout: Duration, now: F) -> Result<Self::ReadGuard<'_>, LockError>
    where
        F: Fn() -> Duration,
    {
        let start = now();
        self.try_read_until(|| now().saturating_sub(start) >= timeout)
    }

    /// Tries to acquire a write lock until the timeout expires.
    fn try_write_until<F>(&self, is_expired: F) -> Result<Self::WriteGuard<'_>, LockError>
    where
        F: Fn() -> bool;

    /// Tries to acquire a write lock for at most `timeout` duration.
    fn try_write_for<F>(&self, timeout: Duration, now: F) -> Result<Self::WriteGuard<'_>, LockError>
    where
        F: Fn() -> Duration,
    {
        let start = now();
        self.try_write_until(|| now().saturating_sub(start) >= timeout)
    }
}
