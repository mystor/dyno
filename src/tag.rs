//! Simple type-based tag values for use in generic code.
//!
//! Additional tags for complex types, such as `MutexGuard<'a, T>`, may be
//! implemented by downstream crates with a `Tag<'a>` impl on the tag type.

use crate::Tag;
use core::marker::PhantomData;

/// Type-based `Tag` for `&'a T` types.
pub struct Ref<T: ?Sized + 'static>(PhantomData<T>);

impl<'a, T: ?Sized + 'static> Tag<'a> for Ref<T> {
    type Type = &'a T;
}

/// Type-based `Tag` for `&'a mut T` types.
pub struct RefMut<T: ?Sized + 'static>(PhantomData<T>);

impl<'a, T: ?Sized + 'static> Tag<'a> for RefMut<T> {
    type Type = &'a mut T;
}

/// Type-based `Tag` for static `T` types.
pub struct Value<T: 'static>(PhantomData<T>);

impl<'a, T: 'static> Tag<'a> for Value<T> {
    type Type = T;
}

/// Tag combinator to wrap the given tag's value in an `Option<T>`
pub struct Optional<I>(PhantomData<I>);

impl<'a, I: Tag<'a>> Tag<'a> for Optional<I> {
    type Type = Option<I::Type>;
}
