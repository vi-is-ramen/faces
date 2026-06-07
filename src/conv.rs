//! Conversion traits and helper functions.

/// A trait for consuming conversions from `Self` to `T`.
pub trait Convertable<T> {
    /// Consumes `self` and produces a value of type `T`.
    fn to(self) -> T;
}

/// A trait for conversions from `&Self` to `T`.
pub trait ConvertableRef<T> {
    /// Converts a reference to `Self` into a value of type `T`.
    fn to(&self) -> T;
}

/// A trait for conversions from `&mut Self` to `T`.
pub trait ConvertableMut<T> {
    /// Converts a mutable reference to `Self` into a value of type `T`.
    fn to(&mut self) -> T;
}

/// Convenience function that performs a consuming conversion.
#[inline]
pub fn to<T, F: Convertable<T>>(from: F) -> T {
    Convertable::<T>::to(from)
}

/// Convenience function that performs a conversion from a shared reference.
#[inline]
pub fn ref_to<T, F: ConvertableRef<T>>(from: &F) -> T {
    ConvertableRef::<T>::to(from)
}

/// Convenience function that performs a conversion from a mutable reference.
#[inline]
pub fn mut_to<T, F: ConvertableMut<T>>(from: &mut F) -> T {
    ConvertableMut::<T>::to(from)
}

/// An unsafe version of [`Convertable`]. Implementors must ensure the conversion
/// is sound under all conditions described in their documentation.
pub unsafe trait UnsafeConvertable<T> {
    /// Unsafely consumes `self` and produces a value of type `T`.
    ///
    /// ## Safety
    /// The implementor must guarantee that the conversion is valid.
    unsafe fn to(self) -> T;
}

/// An unsafe version of [`ConvertableRef`].
pub unsafe trait UnsafeConvertableRef<T> {
    /// Unsafely converts a reference to `Self` into a value of type `T`.
    ///
    /// ## Safety
    /// The implementor must guarantee that the conversion is valid.
    unsafe fn to(&self) -> T;
}

/// An unsafe version of [`ConvertableMut`].
pub unsafe trait UnsafeConvertableMut<T> {
    /// Unsafely converts a mutable reference to `Self` into a value of type `T`.
    ///
    /// ## Safety
    /// The implementor must guarantee that the conversion is valid.
    unsafe fn to(&mut self) -> T;
}

/// Convenience function for [`UnsafeConvertable::to`].
#[inline]
pub unsafe fn unsafe_to<T, F: UnsafeConvertable<T>>(from: F) -> T {
    unsafe { UnsafeConvertable::<T>::to(from) }
}

/// Convenience function for [`UnsafeConvertableRef::to`].
#[inline]
pub unsafe fn unsafe_ref_to<T, F: UnsafeConvertableRef<T>>(from: &F) -> T {
    unsafe { UnsafeConvertableRef::<T>::to(from) }
}

/// Convenience function for [`UnsafeConvertableMut::to`].
#[inline]
pub unsafe fn unsafe_mut_to<T, F: UnsafeConvertableMut<T>>(from: &mut F) -> T {
    unsafe { UnsafeConvertableMut::<T>::to(from) }
}
