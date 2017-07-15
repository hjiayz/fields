extern crate fields_trait;
#[macro_use]
extern crate fields_derive;
use fields_trait::FieldNames;
#[derive(FieldNames)]
struct Test {
    Foo: String,
    Bar: i64,
}
#[test]
fn test_fields() {
    assert_eq!(Test::get_fields(), ["Foo", "Bar"]);
}
