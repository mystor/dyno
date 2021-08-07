//! Tag-based value lookup API for trait objects.
//!
//! This provides a similar API to my `object_provider` crate, built on top of
//! `dyno`.

use crate::tag::Optional;
use crate::tagged::{TagValue, Tagged};
use crate::Tag;

/// Implementation detail shared between `Request<'a>` and `ConcreteRequest<'a, I>`.
///
/// Generally this value is used through the `Request<'a>` type alias as a `&mut
/// Request<'a>` outparameter, or constructed with the `ConcreteRequest<'a, I>`
/// type alias.
#[doc(hidden)]
#[repr(transparent)]
pub struct RequestImpl<T: ?Sized> {
    tagged: T,
}

/// An untyped request for a tagged value of a specific type.
pub type Request<'a> = RequestImpl<dyn Tagged<'a> + 'a>;

/// A concrete request for a tagged value. Can be coerced to `Request<'a>` to be
/// passed to provider methods.
pub type ConcreteRequest<'a, I> = RequestImpl<TagValue<'a, Optional<I>>>;

impl<'a> Request<'a> {
    /// Check if the request is for a value with the given tag `I`. If it is,
    /// returns `true`.
    pub fn is<I>(&self) -> bool
    where
        I: Tag<'a>,
    {
        self.tagged.is::<Optional<I>>()
    }

    /// Attempts to provide a value with the given `Tag` to the request.
    pub fn provide<I>(&mut self, value: I::Type) -> &mut Self
    where
        I: Tag<'a>,
    {
        if let Some(res @ TagValue(None)) = self.tagged.downcast_mut::<Optional<I>>() {
            res.0 = Some(value);
        }
        self
    }

    /// Attempts to provide a value with the given `Tag` to the request.
    pub fn provide_with<I, F>(&mut self, f: F) -> &mut Self
    where
        I: Tag<'a>,
        F: FnOnce() -> I::Type,
    {
        if let Some(res @ TagValue(None)) = self.tagged.downcast_mut::<Optional<I>>() {
            res.0 = Some(f());
        }
        self
    }
}

impl<'a, I> ConcreteRequest<'a, I>
where
    I: Tag<'a>,
{
    /// Construct a new unfulfilled concrete request for the given type. This
    /// can be coerced to a `Request<'a>` to pass to a type-erased provider
    /// method.
    pub fn new() -> Self {
        RequestImpl {
            tagged: TagValue(None),
        }
    }

    /// Take any provided value from this concrete request.
    pub fn take(self) -> Option<I::Type> {
        self.tagged.0
    }
}

/// Trait implemented by a type which can dynamically provide tagged values.
pub trait Provider {
    fn provide<'a>(&'a self, request: &mut Request<'a>);
}

impl dyn Provider {
    /// Request a specific value by a given tag from the `Provider`.
    pub fn request<'a, I>(&'a self) -> Option<I::Type>
    where
        I: Tag<'a>,
    {
        let mut request = <ConcreteRequest<'a, I>>::new();
        self.provide(&mut request);
        request.take()
    }
}
