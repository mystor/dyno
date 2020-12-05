use crate::Tag;
use std::marker::PhantomData;

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
