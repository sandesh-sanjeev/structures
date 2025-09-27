//! # Structures
//!
//! Implementation of various heap allocated data structures.
//!
//! ## Safety
//!
//! I could've certainly used no unsafe, but I want to implement everything from scratch.
//! That requires working with raw pointers and memory, so unsafe is (conservatively) used
//! in the crate. As of now Miri is used to test for undefined behavior.
//!
//! ## Array
//!
//! An [`Array`] is a fixed length collection of elements held in contagious memory segment.
//! This is the heap allocated equivalent of a stack allocated array. Since this data structure
//! is heap allocated, length of the array can be provided at runtime.
//!
//! Provides [`MaybeUninitArray`] for the adventurous, which is an array that can hold uninitialized
//! elements. Elements can be safely aliased only after initialization, which the responsibility of
//! the caller and thus requires use of unsafe.

pub(crate) mod array;

pub use array::{Array, MaybeUninitArray};
