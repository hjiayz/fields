use std::vec::Vec;
pub trait FieldNames {
    fn get_fields() -> Vec<&'static str>;
}