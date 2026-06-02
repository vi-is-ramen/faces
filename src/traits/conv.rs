/// Consumes `self` to produce a value of type `T`.
///
/// This trait is analogous to [`Into<T>`] but is defined for owned `self`.
/// Implementations should perform any necessary conversion, potentially
/// consuming the original value in the process.
///
/// # Examples
/// ```
/// use faces::traits::Convertable;
/// struct Wrapper(i32);
///
/// impl Convertable<i32> for Wrapper {
///     fn to(self) -> i32 {
///         self.0
///     }
/// }
///
/// let w = Wrapper(42);
/// let value: i32 = w.to();
/// assert_eq!(value, 42);
/// ```
pub trait Convertable<T> {
    fn to(self) -> T;
}

/// Borrows `self` immutably to produce a value of type `T`.
///
/// This trait is similar to [`Convertable<T>`] but takes `&self` instead of
/// `self`, allowing the original value to be reused after conversion.
/// Implementations should not modify the borrowed data.
///
/// # Examples
/// ```
/// use faces::traits::ConvertableRef;
/// struct Display(i32);
///
/// impl ConvertableRef<String> for Display {
///     fn to(&self) -> String {
///         format!("Value: {}", self.0)
///     }
/// }
///
/// let d = Display(100);
/// let s = d.to(); // `d` is still usable here
/// assert_eq!(s, "Value: 100");
/// ```
pub trait ConvertableRef<T> {
    fn to(&self) -> T;
}

/// Borrows `self` mutably to produce a value of type `T`.
///
/// This trait takes `&mut self`, allowing implementations to modify the
/// original value during conversion. Useful when the conversion requires
/// internal state changes or avoids unnecessary cloning.
///
/// # Examples
/// ```
/// use faces::traits::ConvertableMut;
/// struct Accumulator(Vec<i32>);
///
/// impl ConvertableMut<i32> for Accumulator {
///     fn to(&mut self) -> i32 {
///         self.0.drain(..).sum()
///     }
/// }
///
/// let mut acc = Accumulator(vec![1, 2, 3]);
/// let sum: i32 = acc.to(); // `acc` is now empty
/// assert_eq!(sum, 6);
/// ```
pub trait ConvertableMut<T> {
    fn to(&mut self) -> T;
}

/// Convenience function to emulate UFCS for [`Convertable::to`].
///
/// This function allows you to call `to()` on a value without needing to import
/// the trait or qualify the method call. It simply forwards to the trait method.
/// Useful in generic contexts or when you want to avoid repetitive trait imports.
///
/// # Type Parameters
/// * `T` – The target type to convert into.
/// * `F` – The source type that implements [`Convertable<T>`].
///
/// # Examples
/// ```
/// use faces::traits::{Convertable, to};
/// struct Wrapper(i32);
///
/// impl Convertable<i32> for Wrapper {
///     fn to(self) -> i32 { self.0 }
/// }
///
/// let w = Wrapper(42);
/// let value: i32 = to(w);
/// assert_eq!(value, 42);
/// ```
#[inline]
pub fn to<T, F: Convertable<T>>(from: F) -> T {
    Convertable::<T>::to(from)
}

/// Convenience function to emulate UFCS for [`ConvertableRef::to`].
///
/// This function allows you to call `to()` on a shared reference without needing
/// to import the trait. It forwards to [`ConvertableRef::to`], performing an
/// immutable borrow conversion.
///
/// # Type Parameters
/// * `T` – The target type to convert into.
/// * `F` – The source type that implements [`ConvertableRef<T>`].
///
/// # Examples
/// ```
/// use faces::traits::{ConvertableRef, ref_to};
/// struct Display(i32);
///
/// impl ConvertableRef<String> for Display {
///     fn to(&self) -> String { format!("Value: {}", self.0) }
/// }
///
/// let d = Display(100);
/// let s = ref_to(&d);
/// assert_eq!(s, "Value: 100");
/// // `d` is still usable here
/// ```
#[inline]
pub fn ref_to<T, F: ConvertableRef<T>>(from: &F) -> T {
    ConvertableRef::<T>::to(from)
}

/// Convenience function to emulate UFCS for [`ConvertableMut::to`].
///
/// This function allows you to call `to()` on a mutable reference without needing
/// to import the trait. It forwards to [`ConvertableMut::to`], allowing the
/// conversion to mutate the source.
///
/// # Type Parameters
/// * `T` – The target type to convert into.
/// * `F` – The source type that implements [`ConvertableMut<T>`].
///
/// # Examples
/// ```
/// use faces::traits::{ConvertableMut, mut_to};
/// struct Accumulator(Vec<i32>);
///
/// impl ConvertableMut<i32> for Accumulator {
///     fn to(&mut self) -> i32 { self.0.drain(..).sum() }
/// }
///
/// let mut acc = Accumulator(vec![1, 2, 3]);
/// let sum = mut_to(&mut acc);
/// assert_eq!(sum, 6);
/// // `acc` has been mutated (its vector is now empty)
/// ```
#[inline]
pub fn mut_to<T, F: ConvertableMut<T>>(from: &mut F) -> T {
    ConvertableMut::<T>::to(from)
}

// ===== Unsafe Conversion Traits =====

/// Unsafe version of [`Convertable`] that consumes `self` to produce a value of type `T`.
///
/// This trait is identical to [`Convertable`] but requires an `unsafe` impl and call.
/// The safety contract is implementation-defined; implementors must document
/// any preconditions (e.g., validity of internal pointers, memory layout requirements).
///
/// # Safety
/// Implementing this trait is `unsafe` because the `to` method may have safety
/// invariants that the caller must uphold. For example, it might dereference raw
/// pointers or assume certain bit patterns are valid.
///
/// Calling `unsafe fn to(self)` is also `unsafe` – the caller must ensure that
/// any preconditions documented by the implementation are satisfied.
///
/// # Examples
/// ```
/// use faces::traits::UnsafeConvertable;
///
/// struct Wrapper(*mut i32);
///
/// unsafe impl UnsafeConvertable<i32> for Wrapper {
///     unsafe fn to(self) -> i32 {
///         *self.0 // Unsafe dereference
///     }
/// }
///
/// let mut x = 42;
/// let w = Wrapper(&mut x);
/// unsafe {
///     let value = w.to();
///     assert_eq!(value, 42);
/// }
/// ```
pub unsafe trait UnsafeConvertable<T> {
    /// Performs the unsafe conversion, consuming `self`.
    ///
    /// # Safety
    /// The caller must guarantee that any safety conditions required by the
    /// implementation are met (e.g., valid pointers, correct alignment,
    /// proper initialization).
    unsafe fn to(self) -> T;
}

/// Unsafe version of [`ConvertableRef`] that borrows `self` immutably.
///
/// This trait allows conversion from `&self` to `T`, but the conversion may
/// involve unsafe operations (e.g., reading from raw pointers). The safety
/// contract is implementation-defined.
///
/// # Safety
/// Implementing this trait is `unsafe` because the `to` method may have
/// invariants that callers must uphold. Calling the method is also `unsafe`.
pub unsafe trait UnsafeConvertableRef<T> {
    /// Performs the unsafe conversion from an immutable reference.
    ///
    /// # Safety
    /// Caller must ensure all preconditions documented by the implementation
    /// are satisfied (e.g., the borrowed data is valid for the entire operation,
    /// no data races, etc.).
    unsafe fn to(&self) -> T;
}

/// Unsafe version of [`ConvertableMut`] that borrows `self` mutably.
///
/// This trait allows conversion from `&mut self` to `T`, potentially modifying
/// the source via unsafe code. Implementations may rely on internal mutability
/// or raw pointers.
///
/// # Safety
/// Implementing this trait is `unsafe` because the `to` method may require
/// exclusive access or other invariants. Calling it is also `unsafe`.
pub unsafe trait UnsafeConvertableMut<T> {
    /// Performs the unsafe conversion from a mutable reference.
    ///
    /// # Safety
    /// Caller must guarantee that any preconditions (e.g., no other references
    /// to the data, correct alignment) are satisfied.
    unsafe fn to(&mut self) -> T;
}

// ===== Unsafe UFCS Emulation Functions =====

/// Convenience function to emulate UFCS for [`UnsafeConvertable::to`].
///
/// This allows calling `unsafe_to(value)` instead of
/// `UnsafeConvertable::<T>::to(value)`. The call is unsafe and forwards directly.
///
/// # Safety
/// Same as [`UnsafeConvertable::to`] – caller must ensure all preconditions
/// documented by the implementation of `F` for `T` are met.
#[inline]
pub unsafe fn unsafe_to<T, F: UnsafeConvertable<T>>(from: F) -> T {
    unsafe { UnsafeConvertable::<T>::to(from) }
}

/// Convenience function to emulate UFCS for [`UnsafeConvertableRef::to`].
///
/// Equivalent to `UnsafeConvertableRef::<T>::to(from)`. Call is unsafe.
///
/// # Safety
/// Same as [`UnsafeConvertableRef::to`] – caller must uphold the implementation's
/// safety requirements.
#[inline]
pub unsafe fn unsafe_ref_to<T, F: UnsafeConvertableRef<T>>(from: &F) -> T {
    unsafe { UnsafeConvertableRef::<T>::to(from) }
}

/// Convenience function to emulate UFCS for [`UnsafeConvertableMut::to`].
///
/// Equivalent to `UnsafeConvertableMut::<T>::to(from)`. Call is unsafe.
///
/// # Safety
/// Same as [`UnsafeConvertableMut::to`] – caller must uphold the implementation's
/// safety requirements.
#[inline]
pub unsafe fn unsafe_mut_to<T, F: UnsafeConvertableMut<T>>(from: &mut F) -> T {
    unsafe { UnsafeConvertableMut::<T>::to(from) }
}
