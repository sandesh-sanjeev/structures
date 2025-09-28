//! # Structures
//!
//! Implementation of various heap allocated data structures.
//!
//! ## Safety
//!
//! The goal is to implement everything from scratch. That requires working with raw
//! pointers and memory. So unsafe is (conservatively) used in the crate. As of now
//! Miri is used to test for undefined behavior.
//!
//! ## Array
//!
//! An [`Array`] is a fixed length collection of elements held in contagious memory segment.
//! This is the heap allocated equivalent of a stack allocated array. Since this data structure
//! is heap allocated, length of the array can be provided at runtime.
//!
//! ## LazyArray
//!
//! A [`LazyArray`] is an array that can hold uninitialized elements. Elements can be safely aliased
//! only after initialization, which the responsibility of the caller and thus requires use of unsafe.
//! Additionally caller is responsible for dropping initialized values (which also requires unsafe).
//!
//! ## RingArray
//!
//! A [`RingArray`] is a simple ring buffer that uses a [`LazyArray`] to store elements.

pub(crate) mod array;

pub use array::ring::RingArray;
pub use array::{Array, LazyArray};

#[cfg(test)]
pub(crate) mod tests {
    use bolero::TypeGenerator;

    /// Test with sized trivially droppable type.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, TypeGenerator)]
    pub(crate) struct Seqno(u64);

    /// Test with type that is not trivially droppable.
    #[derive(Debug, Clone, PartialEq, Eq, TypeGenerator)]
    pub(crate) struct Bytes(Vec<u8>);

    /// Test with zero sized type.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, TypeGenerator)]
    pub(crate) struct Zst;
}
