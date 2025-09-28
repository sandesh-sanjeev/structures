//! A ring buffer backed by an Array.

use crate::{Array, LazyArray};

/// A ring buffer that uses a [`LazyArray`] for storage.
///
/// Works like any other ring buffer. You append elements into one end of the
/// ring buffer. Elements are evicted from the other end of the ring buffer to
/// reclaim space for new ones.
pub struct RingArray<T> {
    len: usize,
    next: usize,
    array: LazyArray<T>,
}

impl<T> RingArray<T> {
    /// Create a ring buffer with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of elements ring buffer can hold.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            len: 0,
            next: 0,
            array: Array::lazy(capacity),
        }
    }

    /// Reference to elements held as a pair of slices.
    ///
    /// Elements are ordered based on the insertion order. What slice holds
    /// how many elements are undefined. The only guarantee is that elements
    /// are ordered across slices.
    #[inline]
    pub fn as_slices(&self) -> (&[T], &[T]) {
        // If the ring buffer has not wrapper around, it doesn't have a tail.
        // Everything is just one contiguous sequence of elements.
        let cap = self.array.len();
        if self.len < cap {
            let head = unsafe { self.array.assume_init(0, self.len) };
            return (head, Default::default());
        }

        unsafe {
            let head = self.array.assume_init(self.next, cap - self.next);
            let tail = self.array.assume_init(0, self.next);
            (head, tail)
        }
    }

    /// Iterator to iterate through contents of the ring buffer.
    ///
    /// Elements are ordered based on the insertion order of elements.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let (head, tail) = self.as_slices();
        head.iter().chain(tail.iter())
    }
}

impl<T: Copy> RingArray<T> {
    /// Append elements into the ring buffer.
    ///
    /// If the ring buffer is at capacity, space will be reclaimed by
    /// trimming oldest element(s).
    ///
    /// # Arguments
    ///
    /// * `elems` - Elements to append into ring buffer.
    #[inline]
    pub fn copy_from_slice(&mut self, elems: &[T]) {
        // Bail early if there is nothing to do.
        if self.array.is_empty() || elems.is_empty() {
            return;
        }

        // Skip elements that will never be visible in ring buffer.
        let cap = self.array.len();
        let start = elems.len().saturating_sub(cap);

        // Copy elements into the ring buffer.
        // If elements reach end of the ring buffer, we have to wrap around.
        match elems[start..].split_at_checked(cap - self.next) {
            None => {
                self.array.copy_from_slice(self.next, elems);
                self.next += elems.len();
                self.len = std::cmp::min(self.len + elems.len(), cap);
            }

            Some((head, tail)) => {
                self.array.copy_from_slice(self.next, head);
                self.array.copy_from_slice(0, tail);
                self.next = tail.len();
                self.len = cap;
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{Seqno, Zst};
    use bolero::{check, generator};
    use pastey::paste;
    use std::collections::VecDeque;

    // Maximum size of inputs in property based tests.
    const MAX_SIZE: usize = 1024 * 1024; // 1 MB

    struct Oracle<T> {
        cap: usize,
        deque: VecDeque<T>,
    }

    impl<T> Oracle<T> {
        fn with_capacity(capacity: usize) -> Self {
            Self {
                cap: capacity,
                deque: VecDeque::with_capacity(capacity),
            }
        }

        fn iter(&self) -> impl Iterator<Item = &T> {
            self.deque.iter()
        }
    }

    impl<T: Clone> Oracle<T> {
        fn extend_from_slice(&mut self, items: &[T]) {
            if self.cap == 0 {
                return;
            }

            for item in items {
                if self.deque.len() == self.cap {
                    self.deque.pop_front();
                }

                self.deque.push_back(item.clone());
            }
        }
    }

    macro_rules! test_ring {
        ($($type:ty),*) => {
            paste! {
                $(
                    #[test]
                    fn [<test_ring_array_ $type:snake>]() {
                        check!()
                            .with_max_len(MAX_SIZE)
                            .with_generator((
                                generator::produce::<usize>().with().bounds(0..1024),
                                generator::produce::<Vec<Vec<$type>>>(),
                            ))
                            .for_each(|(capacity, operations)| {
                                // Ring buffers for equivalence testing.
                                let mut ring = RingArray::with_capacity(*capacity);
                                let mut oracle = Oracle::with_capacity(*capacity);

                                // Process the batch of items.
                                for items in operations {
                                    // Copy the batch of items into the ring buffer.
                                    ring.copy_from_slice(items);
                                    oracle.extend_from_slice(items);

                                    // Make sure items are the same between the ring buffers.
                                    let ring_items: Vec<_> = ring.iter().collect();
                                    let oracle_items: Vec<_> = oracle.iter().collect();
                                    assert_eq!(ring_items, oracle_items);
                                }
                            });
                    }
                )*
            }
        };
    }

    test_ring!(Zst, Seqno);
}
