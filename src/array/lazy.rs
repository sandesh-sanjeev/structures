//! Extension to make working with LazyArray easier.

use super::LazyArray;
use std::{
    mem::{MaybeUninit, transmute},
    ptr::drop_in_place,
};

impl<T> LazyArray<T> {
    /// Get reference to an slice of elements assuming they were initialized.
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure elements are indeed initialized.
    ///
    /// # Panic
    ///
    /// Panics if reads beyond end of the array.
    ///
    /// # Arguments
    ///
    /// * `index` - Starting index (inclusive) of the slice.
    /// * `len` - Number of elements including the starting index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use structures::Array;
    /// let mut array = Array::lazy(10);
    ///
    /// // Copy elements into the array.
    /// let elems: Vec<_> = (0..10).collect();
    /// array.copy_from_slice(0, &elems);
    ///
    /// // Make sure copied correctly.
    /// assert_eq!(unsafe { array.assume_init(0, elems.len()) }, &elems);
    /// ```
    #[inline]
    pub unsafe fn assume_init(&self, index: usize, len: usize) -> &[T] {
        // Safety: It is the responsibility of the caller to ensure memory
        // is actually initialized in the given range. T has the same size
        // and alignment as MaybeUninit<T>.
        unsafe { transmute(&self[index..index + len]) }
    }

    /// Get a mutable reference to an slice of elements assuming they were initialized.
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure elements are indeed initialized.
    ///
    /// # Panic
    ///
    /// Panics if reads beyond end of the array.
    ///
    /// # Arguments
    ///
    /// * `index` - Starting index (inclusive) of the slice.
    /// * `len` - Number of elements including the starting index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use structures::Array;
    /// let mut array = Array::lazy(10);
    ///
    /// // Initialize elements.
    /// let elems: Vec<_> = (0..10).map(|_| vec![1, 2, 3]).collect();
    /// array.write_from_slice(0, &elems);
    ///
    /// // Mutate for elements.
    /// for elem in unsafe { array.assume_init_mut(0, elems.len()) } {
    ///     elem.clear();
    /// }
    ///
    /// // Make sure array was mutated correctly.
    /// let elems: Vec<_> = (0..10).map(|_| vec![]).collect();
    /// assert_eq!(unsafe { array.assume_init(0, elems.len()) }, &elems);
    ///
    /// // Don't forget to drop elements!
    /// unsafe { array.assume_init_drop(0, elems.len()) };
    /// ```
    #[inline]
    pub unsafe fn assume_init_mut(&mut self, index: usize, len: usize) -> &mut [T] {
        // Safety: It is the responsibility of the caller to ensure memory
        // is actually initialized in the given range. T has the same size
        // and alignment as MaybeUninit<T>.
        unsafe { transmute(&mut self[index..index + len]) }
    }

    /// Drop a slice of elements assuming they were initialized.
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure elements are indeed initialized.
    ///
    /// # Panic
    ///
    /// Panics if reads beyond end of the array.
    ///
    /// # Arguments
    ///
    /// * `index` - Starting index (inclusive) of the slice.
    /// * `len` - Number of elements including the starting index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use structures::Array;
    /// let mut array = Array::lazy(10);
    ///
    /// // Initialize elements.
    /// let elems: Vec<_> = (0..10).map(|_| vec![11, 72, 93]).collect();
    /// array.write_from_slice(0, &elems);
    ///
    /// // Make sure elements are dropped.
    /// unsafe { array.assume_init_drop(0, elems.len()) };
    /// ```
    #[inline]
    pub unsafe fn assume_init_drop(&mut self, index: usize, len: usize) {
        // Safety: It is the responsibility of the caller to ensure memory
        // is actually initialized in the given range.
        unsafe {
            let to_drop = self.assume_init_mut(index, len);
            drop_in_place(to_drop);
        }
    }
}

impl<T: Clone> LazyArray<T> {
    /// Initialize a slice of elements with another slice.
    ///
    /// # Safety
    ///
    /// This method does not execute `drop` on the slice of elements in array.
    /// This might result in resource leaks in certain cases. To execute drop
    /// before write see [`LazyArray::overwrite_from_slice`] instead.
    ///
    /// Rust does not consider resource leaks an unsafe operation, so this method
    /// is not marked as unsafe.
    ///
    /// # Panic
    ///
    /// Panics if reads beyond end of the array.
    ///
    /// # Arguments
    ///
    /// * `index` - Starting index (inclusive) of the slice.
    /// * `elems` - Elements to overwrite slice with.
    ///
    /// # Examples
    ///
    /// ```
    /// # use structures::Array;
    /// let mut array = Array::lazy(10);
    ///
    /// // Initialize elements.
    /// let elems: Vec<_> = (0..10).map(|_| vec![1, 2, 3]).collect();
    /// assert_eq!(array.write_from_slice(0, &elems), &elems);
    ///
    /// // Don't forget to drop elements!
    /// unsafe { array.assume_init_drop(0, elems.len()) };
    /// ```
    #[inline]
    pub fn write_from_slice(&mut self, index: usize, elems: &[T]) -> &mut [T] {
        for (dst, src) in self[index..index + elems.len()].iter_mut().zip(elems) {
            dst.write(src.clone());
        }

        // Safety: We just initialized these elements.
        unsafe { transmute(self.assume_init_mut(index, elems.len())) }
    }

    /// Overwrite a slice of elements with another slice.
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure elements are indeed initialized.
    ///
    /// # Panic
    ///
    /// Panics if reads beyond end of the array.
    ///
    /// # Arguments
    ///
    /// * `index` - Starting index (inclusive) of the slice.
    /// * `elems` - Elements to overwrite slice with.
    ///
    /// # Examples
    ///
    /// ```
    /// # use structures::Array;
    /// let mut array = Array::lazy(10);
    ///
    /// // First initialize elements.
    /// let elems: Vec<_> = (0..10).map(|_| vec![1, 2, 3]).collect();
    /// assert_eq!(array.write_from_slice(0, &elems), &elems);
    ///
    /// // Then overwrite them.
    /// let elems: Vec<_> = (0..10).map(|_| vec![4, 5, 6]).collect();
    /// assert_eq!(unsafe { array.overwrite_from_slice(0, &elems) }, &elems);
    ///
    /// // Don't forget to drop elements!
    /// unsafe { array.assume_init_drop(0, elems.len()) };
    /// ```
    #[inline]
    pub unsafe fn overwrite_from_slice(&mut self, index: usize, elems: &[T]) -> &mut [T] {
        for (dst, src) in self[index..index + elems.len()].iter_mut().zip(elems) {
            // Safety: It is the responsibility of the caller to ensure memory
            // is actually initialized in the given range.
            unsafe { dst.assume_init_drop() };

            dst.write(src.clone());
        }

        // Safety: We just initialized these elements.
        unsafe { transmute(self.assume_init_mut(index, elems.len())) }
    }
}

impl<T: Copy> LazyArray<T> {
    /// Copy elements from a slice into the array.
    ///
    /// # Panic
    ///
    /// * If the slice overflows bounds of the array.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to start writes.
    /// * `elems` - Elements to copy into array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use structures::Array;
    /// let mut array = Array::lazy(10);
    ///
    /// // Copy elements into the array.
    /// let elems: Vec<_> = (0..10).collect();
    /// assert_eq!(array.copy_from_slice(0, &elems), &elems);
    /// assert_eq!(unsafe { array.assume_init(0, elems.len()) }, &elems);
    /// ```
    #[inline]
    pub fn copy_from_slice(&mut self, index: usize, elems: &[T]) -> &mut [T] {
        let end_index = index + elems.len();

        // Safety: T has the same size and alignment as MaybeUnint<T>.
        let src: &[MaybeUninit<T>] = unsafe { transmute(elems) };
        self[index..end_index].copy_from_slice(src);

        // Safety:
        // * Index of elements have definitely been initialized.
        // * T has the same size and alignment as MaybeUnint<T>.
        unsafe { transmute(&mut self[index..end_index]) }
    }
}
