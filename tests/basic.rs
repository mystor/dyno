use dyno::{
    tagged::{TagValue, Tagged},
    Tag,
};

#[derive(Debug)]
struct Status<'a> {
    value: &'a str,
}

struct StatusTag;

impl<'a> Tag<'a> for StatusTag {
    type Type = Status<'a>;
}

fn get_thing<'a>(value: &'a str) -> Box<dyn Tagged<'a> + 'a> {
    Box::new(TagValue::<StatusTag>(Status { value }))
}

#[test]
fn use_get_thing() {
    let value = String::from("hello, world");
    let tagged = get_thing(&value);
    let downcast = tagged.downcast_box::<StatusTag>();
    assert!(downcast.is_ok());
    if let Ok(status) = downcast {
        assert_eq!(status.0.value, "hello, world");
    }
}
