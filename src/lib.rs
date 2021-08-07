#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod provider;
pub mod tag;
pub mod tagged;

/// This trait is implemented by specific `Tag` types in order to allow
/// describing a type which can be requested for a given lifetime `'a`.
///
/// A few example implementations for type-driven `Tag`s can be found in the
/// [`tag`] module, although crates may also implement their own tags for more
/// complex types with internal lifetimes.
pub trait Tag<'a>: Sized + 'static {
    /// The type of values which may be tagged by this `Tag` for the given
    /// lifetime.
    type Type: 'a;
}
