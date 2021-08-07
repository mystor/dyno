//! Core low-level `Tagged` abstraction.
//!
//! A `dyn Tagged<'a>` can be thought of as similar to `dyn Any`, in that it is
//! a type-erased value of a specific type. The main difference is that a `dyn
//! Tagged<'a>` can contain non-`'static` values due to its use of a `Tag`.
//!
//! A `Tag` is associated with a value by wrapping that value in a `TagValue<'a,
//! I>`. This wrapped type can then be coerced into a `dyn Tagged<'a>`, which
//! can be passed around and downcast using the `downcast_{ref,mut}` methods on
//! the trait object.

// All unsafe code in `dyno` is localized within this module, which provides the
// core safe abstraction.
#![allow(unsafe_code)]

use crate::Tag;
use core::any::TypeId;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

mod private {
    pub trait Sealed {}
}

/// Sealed trait representing a type-erased tagged object.
///
/// This trait is exclusively implemented by the `TagValue` type, and cannot be
/// implemented outside of this crate due to being sealed.
pub unsafe trait Tagged<'a>: private::Sealed + 'a {
    /// The `TypeId` of the `Tag` this value was tagged with.
    fn tag_id(&self) -> TypeId;
}

/// A concrete tagged value for a given tag `I`.
///
/// This is the only type which implements the `Tagged` trait, and encodes
/// additional information about the specific `Tag` into the type. This allows
/// for multiple different tags to support overlapping value ranges, for
/// example, both the `Ref<str>` and `Value<&'static str>` tags can be used to
/// tag a value of type `&'static str`.
#[repr(transparent)]
pub struct TagValue<'a, I: Tag<'a>>(pub I::Type);

impl<'a, I: Tag<'a>> private::Sealed for TagValue<'a, I> where I: Tag<'a> {}

unsafe impl<'a, I> Tagged<'a> for TagValue<'a, I>
where
    I: Tag<'a>,
{
    fn tag_id(&self) -> TypeId {
        TypeId::of::<I>()
    }
}

macro_rules! tagged_methods {
    ($($T:ty),*) => {$(
        impl<'a> $T {
            /// Returns `true` if the dynamic type is tagged with `I`.
            #[inline]
            pub fn is<I>(&self) -> bool
            where
                I: Tag<'a>,
            {
                self.tag_id() == TypeId::of::<I>()
            }

            /// Returns some reference to the dynamic value if it is tagged with `I`,
            /// or `None` if it isn't.
            #[inline]
            pub fn downcast_ref<I>(&self) -> Option<&TagValue<'a, I>>
            where
                I: Tag<'a>,
            {
                if self.is::<I>() {
                    // SAFETY: Just checked whether we're pointing to a
                    // `TagValue<'a, I>`.
                    unsafe { Some(&*(self as *const Self as *const TagValue<'a, I>)) }
                } else {
                    None
                }
            }

            /// Returns some reference to the dynamic value if it is tagged with `I`,
            /// or `None` if it isn't.
            #[inline]
            pub fn downcast_mut<I>(&mut self) -> Option<&mut TagValue<'a, I>>
            where
                I: Tag<'a>,
            {
                if self.is::<I>() {
                    // SAFETY: Just checked whether we're pointing to a
                    // `TagValue<'a, I>`.
                    unsafe { Some(&mut *(self as *mut Self as *mut TagValue<'a, I>)) }
                } else {
                    None
                }
            }

            #[inline]
            #[cfg(feature = "alloc")]
            pub fn downcast_box<I>(self: Box<Self>) -> Result<Box<TagValue<'a, I>>, Box<Self>>
            where
                I: Tag<'a>,
            {
                if self.is::<I>() {
                    unsafe {
                        // SAFETY: Just checked whether we're pointing to a
                        // `TagValue<'a, I>`.
                        let raw: *mut dyn Tagged<'a> = Box::into_raw(self);
                        Ok(Box::from_raw(raw as *mut TagValue<'a, I>))
                    }
                } else {
                    Err(self)
                }
            }
        }
    )*};
}

tagged_methods!(
    dyn Tagged<'a>,
    dyn Tagged<'a> + Send,
    dyn Tagged<'a> + Sync,
    dyn Tagged<'a> + Send + Sync
);
