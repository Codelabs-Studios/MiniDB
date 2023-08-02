use hashbrown::HashMap;
use std::{fs::File, path::Path};

use crate::{stores::container, values::key::Key};

use super::container::Container;

pub struct Database {
    pub filename: String,
    pub containers: Vec<HashMap<String, Container>>,
}

impl Database {
    pub(crate) fn new(filename: String) -> Database {
        if !Path::new(&filename).exists() {
            File::create(&filename).expect("Unable to Database create file");
        }

        Database {
            filename,
            containers: Vec::new(),
        }
    }

    pub fn get_container(&self, name: &str) -> Option<&Container> {
        for container in &self.containers {
            if let Some(container) = container.get(name) {
                return Some(container);
            }
        }

        None
    }

    pub fn create_container(&mut self, name: String) {
        let mut container = HashMap::new();
        container.insert(name.clone(), Container::new(name));
        self.containers.push(container);
    }
}
