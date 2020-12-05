use dyno::{Tag, Tagged};
use std::marker::PhantomData;

#[derive(Debug)]
struct Status<'a> {
    value: &'a str,
}

struct StatusTag;

impl<'a> Tag<'a> for StatusTag {
    type Type = Status<'a>;
}

fn get_thing<'a>(value: &'a str) -> Box<dyn Tagged<'a> + 'a> {
    Tagged::tag_box::<StatusTag>(Box::new(Status { value }))
}

#[test]
fn use_get_thing() {
    let value = String::from("hello, world");
    let tagged = get_thing(&value);
    let downcast = tagged.downcast_box::<StatusTag>();
    assert!(downcast.is_ok());
    if let Ok(status) = downcast {
        assert_eq!(status.value, "hello, world");
    }
}

struct RefRequest<T: ?Sized>(PhantomData<T>);
impl<'a, T: ?Sized + 'static> Tag<'a> for RefRequest<T> {
    type Type = Option<&'a T>;
}

trait ObjectProvider {
    fn provide<'a, 'b>(&'a self, out: &'b mut (dyn Tagged<'a> + 'a));
}

struct MyType {
    field: String,
}

impl ObjectProvider for MyType {
    fn provide<'a, 'b>(&'a self, out: &'b mut (dyn Tagged<'a> + 'a)) {
        let x: Option<&mut Option<&'a str>> = out.downcast_mut::<RefRequest<str>>();
        if let Some(req @ None) = x {
            *req = Some(&self.field[..]);
        }
    }
}

#[test]
fn use_object_provider() {
    let my_object_provider: Box<dyn ObjectProvider> = Box::new(MyType {
        field: "hello, jane!".to_owned(),
    });

    let mut result: Option<&str> = None;
    my_object_provider.provide(Tagged::tag_mut::<RefRequest<str>>(&mut result));

    assert_eq!(result, Some("hello, jane!"));
}
