use std::any::Any;
use std::collections::HashMap;

pub struct Key {
    data: HashMap<String, Box<dyn Any>>,
}

impl Key {
    pub fn new() -> Self {
        Key {
            data: HashMap::new(),
        }
    }

    pub fn insert<T: Any + 'static>(&mut self, key: String, value: T) {
        self.data.insert(key, Box::new(value));
    }

    pub fn get<T: Any + 'static>(&self, key: &str) -> Option<&T> {
        if let Some(value) = self.data.get(key) {
            value.downcast_ref::<T>()
        } else {
            None
        }
    }
}
