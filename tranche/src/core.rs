// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::fmt;
use core::marker::PhantomData as marker;
use core::mem;
use core::ptr::NonNull;
use core::slice;

/// A tranche of `T`.
///
/// Tranches are like slices but represented of a start pointer and an end
/// pointer. They are geared towards being mutably slicing from their start or
/// end, and thus don't implement indexing.
pub struct Tranche<'a, T> {
    start: NonNull<T>,
    end: *const T,
    marker: marker<&'a T>,
}
unsafe impl<T> Send for Tranche<'_, T> where T: Sync {}
unsafe impl<T> Sync for Tranche<'_, T> where T: Sync {}

/// A based tranche of `T`.
///
/// Based tranches are just like tranches, with the addition of an `offset`
/// method which returns how many items were taken from the front of
/// the original based tranche returned from `BasedTranche::new`.
pub struct BasedTranche<'a, T> {
    pub(crate) inner: Tranche<'a, T>,
    base: *const T,
}
unsafe impl<T> Send for BasedTranche<'_, T> where T: Sync {}
unsafe impl<T> Sync for BasedTranche<'_, T> where T: Sync {}

/// A tranche of bytes, equipped with many convenience methods.
///
/// This type implements `std::io::Read` and `std::io::BufRead` when the `std`
/// feature is enabled.
pub type BufTranche<'a> = Tranche<'a, u8>;

/// A based tranche of bytes, equipped with many convenience methods.
///
/// This type implements `std::io::Read` and `std::io::BufRead` when the `std`
/// feature is enabled.
pub type BasedBufTranche<'a> = BasedTranche<'a, u8>;

impl<'a, T> Tranche<'a, T> {
    /// Creates a new tranche of `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::Tranche;
    /// let parisien = Tranche::new(&[
    ///     "baguette",
    ///     "jambon",
    ///     "beurre",
    /// ]);
    /// ```
    pub fn new(slice: &'a impl AsRef<[T]>) -> Self {
        let slice = slice.as_ref();
        let start = unsafe { NonNull::new_unchecked(slice.as_ptr() as *mut T) };
        let end = if mem::size_of::<T>() == 0 {
            (start.as_ptr() as *const u8).wrapping_add(slice.len()) as *const T
        } else {
            unsafe { start.as_ptr().add(slice.len()) }
        };
        Self { start, end, marker }
    }

    /// Returns the number of elements in the tranche.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::Tranche;
    /// let a = Tranche::new(&[1, 2, 3]);
    /// assert_eq!(a.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        ptr_distance(self.start.as_ptr() as *const T, self.end)
    }

    /// Returns `true` if the tranche has a length of 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::Tranche;
    /// let a = Tranche::new(&[1, 2, 3]);
    /// assert!(!a.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.start.as_ptr() as *const T == self.end
    }

    /// Takes the first element out of the tranche.
    ///
    /// Returns the first element of `self`, or `Err(_)` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::Tranche;
    /// let mut v = Tranche::new(&[10, 40, 30]);
    /// assert_eq!(v.take_first().unwrap(), &10);
    /// assert_eq!(v.as_slice(), &[40, 30]);
    ///
    /// let mut w = <Tranche<i32>>::new(&[]);
    /// let err = w.take_first().unwrap_err();
    /// assert_eq!(err.needed(), 1);
    /// assert_eq!(err.len(), 0);
    /// ```
    pub fn take_first(&mut self) -> Result<&'a T, UnexpectedEndError> {
        if (*self).is_empty() {
            return Err(UnexpectedEndError { needed: 1, len: 0 });
        }
        unsafe { Ok(&*self.post_inc_start(1)) }
    }

    /// Takes the first `n` elements out of the tranche.
    ///
    /// Returns a new tranche with the first `n` elements of `self`, or
    /// `Err(_)` if it is not long enough.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::Tranche;
    /// let mut v = Tranche::new(&[10, 40, 30]);
    /// assert_eq!(v.take_front(2).unwrap().as_slice(), &[10, 40]);
    /// assert_eq!(v.as_slice(), &[30]);
    ///
    /// let err = v.take_front(3).unwrap_err();
    /// assert_eq!(err.needed(), 3);
    /// assert_eq!(err.len(), 1);
    /// ```
    pub fn take_front(&mut self, n: usize) -> Result<Self, UnexpectedEndError> {
        let len = self.len();
        if n > len {
            return Err(UnexpectedEndError { needed: n, len });
        }
        let start = unsafe { NonNull::new_unchecked(self.post_inc_start(n) as *mut _) };
        let end = self.start.as_ptr();
        let marker = self.marker;
        Ok(Self { start, end, marker })
    }

    /// Views the tranche's buffer as a slice.
    ///
    /// This has the same lifetime as the original buffer, and so the tranche
    /// can continue to be used while this exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::Tranche;
    /// let mut tranche = Tranche::new(&[1, 2, 3]);
    /// assert_eq!(tranche.as_slice(), &[1, 2, 3]);
    ///
    /// assert!(tranche.take_first().is_ok());
    /// assert_eq!(tranche.as_slice(), &[2, 3]);
    /// ```
    pub fn as_slice(&self) -> &'a [T] {
        unsafe { slice::from_raw_parts(self.start.as_ptr() as *const _, self.len()) }
    }

    /// Returns a raw pointer to the tranche's buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::Tranche;
    /// let tranche = Tranche::new(&[1, 2, 3]);
    /// let ptr = tranche.as_ptr();
    /// ```
    pub const fn as_ptr(&self) -> *const T {
        self.start.as_ptr() as *const _
    }

    unsafe fn post_inc_start(&mut self, offset: usize) -> *const T {
        let old = self.start.as_ptr();
        if mem::size_of::<T>() == 0 {
            self.end = (self.end as *const u8).wrapping_sub(offset) as *const T;
        } else {
            self.start = NonNull::new_unchecked(old.add(offset) as *mut _);
        }
        old
    }
}

impl<'a, T> BasedTranche<'a, T> {
    /// Creates a new based tranche of `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::BasedTranche;
    /// let parisien = BasedTranche::new(&[
    ///     "baguette",
    ///     "jambon",
    ///     "beurre",
    /// ]);
    /// ```
    pub fn new(slice: &'a impl AsRef<[T]>) -> Self {
        Tranche::new(slice).into()
    }

    /// Returns the number of elements in the tranche.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::BasedTranche;
    /// let a = BasedTranche::new(&[1, 2, 3]);
    /// assert_eq!(a.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the tranche has a length of 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::BasedTranche;
    /// let a = BasedTranche::new(&[1, 2, 3]);
    /// assert!(!a.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the starting offset of this based tranche.
    pub fn offset(&self) -> usize {
        if mem::size_of::<T>() == 0 {
            ptr_distance(self.inner.end, self.base)
        } else {
            ptr_distance(self.base, self.inner.start.as_ptr() as *const _)
        }
    }

    /// Takes the first element out of the tranche.
    ///
    /// Returns the first element of `self`, or `Err(_)` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::BasedTranche;
    /// let mut v = BasedTranche::new(&[10, 40, 30]);
    /// assert_eq!(v.take_first().unwrap(), &10);
    /// assert_eq!(v.as_slice(), &[40, 30]);
    ///
    /// let mut w = <BasedTranche<i32>>::new(&[]);
    /// let err = w.take_first().unwrap_err();
    /// assert_eq!(err.needed(), 1);
    /// assert_eq!(err.len(), 0);
    /// ```
    pub fn take_first(&mut self) -> Result<&'a T, UnexpectedEndError> {
        self.inner.take_first()
    }

    /// Takes the first `n` elements out of the tranche.
    ///
    /// Returns a new tranche with the first `n` elements of `self`, or
    /// `Err(_)` if it is not long enough.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::BasedTranche;
    /// let mut v = BasedTranche::new(&[10, 40, 30]);
    /// assert_eq!(v.take_front(2).unwrap().as_slice(), &[10, 40]);
    /// assert_eq!(v.as_slice(), &[30]);
    ///
    /// let err = v.take_front(3).unwrap_err();
    /// assert_eq!(err.needed(), 3);
    /// assert_eq!(err.len(), 1);
    /// ```
    pub fn take_front(&mut self, n: usize) -> Result<Self, UnexpectedEndError> {
        let inner = self.inner.take_front(n)?;
        let base = self.base;
        Ok(Self { inner, base })
    }

    /// Views the tranche's buffer as a slice.
    ///
    /// This has the same lifetime as the original buffer, and so the tranche
    /// can continue to be used while this exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::BasedTranche;
    /// let mut tranche = BasedTranche::new(&[1, 2, 3]);
    /// assert_eq!(tranche.as_slice(), &[1, 2, 3]);
    ///
    /// assert!(tranche.take_first().is_ok());
    /// assert_eq!(tranche.as_slice(), &[2, 3]);
    /// ```
    pub fn as_slice(&self) -> &'a [T] {
        self.inner.as_slice()
    }

    /// Returns a raw pointer to the tranche's buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tranche::BasedTranche;
    /// let tranche = BasedTranche::new(&[1, 2, 3]);
    /// let ptr = tranche.as_ptr();
    /// ```
    pub const fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }
}

#[inline(always)]
fn ptr_distance<T>(start: *const T, end: *const T) -> usize {
    let diff = (end as usize).wrapping_sub(start as usize);
    let size = mem::size_of::<T>();
    if size == 0 {
        diff
    } else {
        diff / size
    }
}

impl<T> Clone for Tranche<'_, T> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<T> Clone for BasedTranche<'_, T> {
    fn clone(&self) -> Self {
        let inner = self.inner.clone();
        let base = self.base;
        Self { inner, base }
    }
}

impl<T> Default for Tranche<'_, T> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<T> Default for BasedTranche<'_, T> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<'a, T> From<BasedTranche<'a, T>> for Tranche<'a, T> {
    fn from(based_tranche: BasedTranche<'a, T>) -> Self {
        based_tranche.inner
    }
}

impl<'a, T> From<Tranche<'a, T>> for BasedTranche<'a, T> {
    fn from(tranche: Tranche<'a, T>) -> Self {
        let inner = tranche;
        let base = if mem::size_of::<T>() == 0 {
            inner.end
        } else {
            inner.start.as_ptr() as *const _
        };
        Self { inner, base }
    }
}

impl<T> fmt::Debug for Tranche<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(fmt)
    }
}

impl<T> fmt::Debug for BasedTranche<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(fmt)
    }
}

/// An error signalling that the end of a tranche was reached unexpectedly.
#[derive(Clone, Debug)]
pub struct UnexpectedEndError {
    needed: usize,
    len: usize,
}

impl UnexpectedEndError {
    /// Returns the number of elements that were needed from the tranche for
    /// the operation to succeed.
    pub fn needed(&self) -> usize {
        self.needed
    }

    /// Returns the number of elements that were in the tranche when the
    /// operation failed.
    pub fn len(&self) -> usize {
        self.len
    }
}

impl fmt::Display for UnexpectedEndError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "unexpected end (needed {}, got {})",
            self.needed, self.len,
        )
    }
}
