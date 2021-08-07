//! This is an example of the same API as `barebones_provider`, but using the
//! built-in provider types, and their nicer API.

use dyno::{
    provider::{Provider, Request},
    tag,
};

/// A simple type which implements the `Provider` trait and can provide values
/// and references from shared references.
struct Example(String);

impl Provider for Example {
    fn provide<'a>(&'a self, request: &mut Request<'a>) {
        request
            .provide::<tag::Ref<str>>(&self.0)
            .provide_with::<tag::Value<String>, _>(|| self.0.clone());
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
