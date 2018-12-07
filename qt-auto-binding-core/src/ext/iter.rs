//! Extensions for [`std::iter`]
//!
//! [`std::iter`]: https://doc.rust-lang.org/std/iter/index.html

/// Extension trait for [`std::iter::Iterator`]
///
/// [`std::iter::Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
pub trait IteratorExt
where
    Self: Sized + Iterator,
{
    /// Consumes the iterator and returns only one item
    ///
    /// Returns the next item in the iterator if and only if the iterator contained one element.
    /// Otherwise, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate qt_auto_binding_core;
    /// use qt_auto_binding_core::ext::iter::IteratorExt;
    ///
    /// let data = vec![1];
    /// assert_eq!(data.into_iter().single(), Some(1));
    ///
    /// let data = vec![1, 2, 3];
    /// assert_eq!(data.into_iter().single(), None);
    ///
    /// let data = Vec::<i32>::new();
    /// assert_eq!(data.into_iter().single(), None);
    /// ```
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    fn single(mut self) -> Option<Self::Item> {
        let first = self.next();
        let second = self.next();

        if let (Some(result), None) = (first, second) {
            Some(result)
        } else {
            None
        }
    }
}

impl<T> IteratorExt for T where T: Iterator {}
