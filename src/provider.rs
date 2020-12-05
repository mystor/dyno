use crate::{Tag, Tagged};

/// An untyped request for a value of a specific type.
///
/// This type is generally used as an `&mut Request<'a>` outparameter.
#[repr(transparent)]
pub struct Request<'a> {
    tagged: dyn Tagged<'a> + 'a,
}

impl<'a> Request<'a> {
    /// Helper for performing transmutes as `Request<'a>` has the same layout as
    /// `dyn Tagged<'a> + 'a`, just with a different type!
    ///
    /// This is just to have our own methods on it, and less of the interface
    /// exposed on the `provide` implementation.
    fn wrap_tagged<'b>(t: &'b mut (dyn Tagged<'a> + 'a)) -> &'b mut Self {
        unsafe { &mut *(t as *mut (dyn Tagged<'a> + 'a) as *mut Request<'a>) }
    }

    pub fn is<I>(&self) -> bool
    where
        I: Tag<'a>,
    {
        self.tagged.is::<ReqTag<I>>()
    }

    pub fn provide<I>(&mut self, value: I::Type) -> &mut Self
    where
        I: Tag<'a>,
    {
        if let Some(res @ None) = self.tagged.downcast_mut::<ReqTag<I>>() {
            *res = Some(value);
        }
        self
    }

    pub fn provide_with<I, F>(&mut self, f: F) -> &mut Self
    where
        I: Tag<'a>,
        F: FnOnce() -> I::Type,
    {
        if let Some(res @ None) = self.tagged.downcast_mut::<ReqTag<I>>() {
            *res = Some(f());
        }
        self
    }
}

pub trait Provider {
    fn provide<'a>(&'a self, request: &mut Request<'a>);
}

impl dyn Provider {
    pub fn request<'a, I>(&'a self) -> Option<I::Type>
    where
        I: Tag<'a>,
    {
        request::<I, _>(|request| self.provide(request))
    }
}

pub fn request<'a, I, F>(f: F) -> Option<<I as Tag<'a>>::Type>
where
    I: Tag<'a>,
    F: FnOnce(&mut Request<'a>),
{
    let mut result: Option<<I as Tag<'a>>::Type> = None;
    f(Request::<'a>::wrap_tagged(Tagged::tag_mut::<ReqTag<I>>(
        &mut result,
    )));
    result
}

/// Implementation detail: Specific `Tag` tag used by the `Request` code under
/// the hood.
///
/// Composition of `Tag` types!
struct ReqTag<I>(I);
impl<'a, I: Tag<'a>> Tag<'a> for ReqTag<I> {
    type Type = Option<I::Type>;
}
