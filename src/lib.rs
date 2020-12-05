use std::any::TypeId;

/// Simple type-based tag values for use in generic code.
pub mod tag;

/// Tag-based value lookup API for trait objects.
pub mod provider;

/// An identifier which may be used to tag a specific
pub trait Tag<'a>: Sized + 'static {
    /// The type of values which may be tagged by this `Tag`.
    type Type: 'a;
}

mod private {
    pub trait Sealed {}
}

/// Sealed trait representing a type-erased tagged object.
pub unsafe trait Tagged<'a>: private::Sealed + 'a {
    /// The `TypeId` of the `Tag` this value was tagged with.
    fn ident_type_id(&self) -> TypeId;
}

/// Internal wrapper type with the same representation as a known external type.
#[repr(transparent)]
struct TaggedImpl<'a, I>
where
    I: Tag<'a>,
{
    _value: I::Type,
}

impl<'a, I> private::Sealed for TaggedImpl<'a, I> where I: Tag<'a> {}

unsafe impl<'a, I> Tagged<'a> for TaggedImpl<'a, I>
where
    I: Tag<'a>,
{
    fn ident_type_id(&self) -> TypeId {
        TypeId::of::<I>()
    }
}

// FIXME: This should also handle the cases for `dyn Tagged<'a> + Send`,
// `dyn Tagged<'a> + Send + Sync` and `dyn Tagged<'a> + Sync`...
//
// Should be easy enough to do it with a macro...
impl<'a> dyn Tagged<'a> {
    /// Tag a reference to a concrete type with a given `Tag`.
    ///
    /// This is like an unsizing coercion, but must be performed explicitly to
    /// specify the specific tag.
    pub fn tag_ref<I>(value: &I::Type) -> &dyn Tagged<'a>
    where
        I: Tag<'a>,
    {
        // SAFETY: `TaggedImpl<'a, I>` has the same representation as `I::Type`
        // due to `#[repr(transparent)]`.
        unsafe { &*(value as *const I::Type as *const TaggedImpl<'a, I>) }
    }

    /// Tag a reference to a concrete type with a given `Tag`.
    ///
    /// This is like an unsizing coercion, but must be performed explicitly to
    /// specify the specific tag.
    pub fn tag_mut<I>(value: &mut I::Type) -> &mut dyn Tagged<'a>
    where
        I: Tag<'a>,
    {
        // SAFETY: `TaggedImpl<'a, I>` has the same representation as `I::Type`
        // due to `#[repr(transparent)]`.
        unsafe { &mut *(value as *mut I::Type as *mut TaggedImpl<'a, I>) }
    }

    /// Tag a reference to a concrete type with a given `Tag`.
    ///
    /// This is like an unsizing coercion, but must be performed explicitly to
    /// specify the specific tag.
    pub fn tag_box<I>(value: Box<I::Type>) -> Box<dyn Tagged<'a>>
    where
        I: Tag<'a>,
    {
        // SAFETY: `TaggedImpl<'a, I>` has the same representation as `I::Type`
        // due to `#[repr(transparent)]`.
        unsafe { Box::from_raw(Box::into_raw(value) as *mut TaggedImpl<'a, I>) }
    }

    /// Returns `true` if the dynamic type is tagged with `I`.
    #[inline]
    pub fn is<I>(&self) -> bool
    where
        I: Tag<'a>,
    {
        self.ident_type_id() == TypeId::of::<I>()
    }

    /// Returns some reference to the dynamic value if it is tagged with `I`,
    /// or `None` if it isn't.
    #[inline]
    pub fn downcast_ref<I>(&self) -> Option<&I::Type>
    where
        I: Tag<'a>,
    {
        if self.is::<I>() {
            // SAFETY: Just checked whether we're pointing to a
            // `TaggedImpl<'a, I>`, which was cast to from an `I::Type`.
            unsafe { Some(&*(self as *const dyn Tagged<'a> as *const I::Type)) }
        } else {
            None
        }
    }

    /// Returns some reference to the dynamic value if it is tagged with `I`,
    /// or `None` if it isn't.
    #[inline]
    pub fn downcast_mut<I>(&mut self) -> Option<&mut I::Type>
    where
        I: Tag<'a>,
    {
        if self.is::<I>() {
            // SAFETY: Just checked whether we're pointing to a
            // `TaggedImpl<'a, I>`, which was cast to from an `I::Type`.
            unsafe { Some(&mut *(self as *mut dyn Tagged<'a> as *mut I::Type)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_box<I>(self: Box<Self>) -> Result<Box<I::Type>, Box<Self>>
    where
        I: Tag<'a>,
    {
        if self.is::<I>() {
            unsafe {
                // SAFETY: Just checked whether we're pointing to a
                // `TaggedImpl<'a, I>`, which was cast to from an `I::Type`.
                let raw: *mut dyn Tagged<'a> = Box::into_raw(self);
                Ok(Box::from_raw(raw as *mut I::Type))
            }
        } else {
            Err(self)
        }
    }
}
