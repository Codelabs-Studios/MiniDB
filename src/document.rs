use crate::key;

#[derive(Debug)]
pub struct Document {
    id: i32,
    keys: Vec<key::Key>,
}
