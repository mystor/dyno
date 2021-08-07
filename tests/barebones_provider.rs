//! This test shows an example of a `Provider` system using only the bare
//! `Tagged` types, to give a more clear idea of what the `provider` module is
//! doing. The `provider` module largely exists as a nicer UX on top of these
//! primitives.

use dyno::{
    tag,
    tagged::{TagValue, Tagged},
    Tag,
};

pub trait Provider {
    fn provide<'a>(&'a self, request: &mut (dyn Tagged<'a> + 'a));
}

/// A simple type which implements the `Provider` trait and can provide values
/// and references from shared references.
struct Example(String);

impl Provider for Example {
    // The part which is quite unergonomic here is the `provide` part. This is
    // the main thing which is changed in the provided `tagged` library by using
    // a wrapper type around `dyn Tagged<'a> + 'a` which provides nicer-to-use
    // methods.
    fn provide<'a>(&'a self, request: &mut (dyn Tagged<'a> + 'a)) {
        // downcasting to a `tag::Ref<str>` - this is done inside of a
        // `tag::Optional` as the result is stored as `Option<&'a str>`.
        if let Some(x) = request.downcast_mut::<tag::Optional<tag::Ref<str>>>() {
            x.0 = Some(&self.0[..]);
        }

        // same as above, but using a concrete `String` type as the downcast target.
        if let Some(x) = request.downcast_mut::<tag::Optional<tag::Value<String>>>() {
            x.0 = Some(self.0.clone());
        }
    }
}

impl dyn Provider {
    fn request<'a, I: Tag<'a>>(&'a self) -> Option<I::Type> {
        let mut result = TagValue::<'a, tag::Optional<I>>(None);
        self.provide(&mut result);
        result.0
    }
}

#[test]
fn request_from_example() {
    let example = Example("hello, world!".to_string());
    let as_provider: &dyn Provider = &example;

    assert_eq!(
        as_provider.request::<tag::Ref<str>>(),
        Some("hello, world!")
    );
    assert_eq!(
        as_provider.request::<tag::Value<String>>(),
        Some("hello, world!".to_string())
    );
}
