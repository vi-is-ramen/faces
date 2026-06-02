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
pub fn mut_to<T, F: ConvertableMut<T>>(from: &mut F) -> T {
    ConvertableMut::<T>::to(from)
}
