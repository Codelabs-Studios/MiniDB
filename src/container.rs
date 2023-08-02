use hashbrown::HashMap;

use crate::key::Key;

pub struct Container {
    name: String,
    documents: Vec<HashMap<String, Key>>,
}

/*
Container {
    "TestContainer",
    {
        aaron: {
            name: "Aaron",
            age: 20,
        },
        bob: {
            name: "Bob",
            age: 21,
        },
        carl: {
            name: "Carl",
            age: 22,
        },
    }
}
*/

impl Container {
    pub fn new(name: String) -> Self {
        Container {
            name,
            documents: Vec::new(),
        }
    }

    pub fn add_document(&mut self, name: String, key: Key) {
        let mut document = HashMap::new();
        document.insert(name, key);
        self.documents.push(document);
    }

    pub fn get_document(&self, index: usize) -> Option<&HashMap<String, Key>> {
        self.documents.get(index)
    }
}
