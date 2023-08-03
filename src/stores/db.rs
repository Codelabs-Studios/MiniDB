use std::{fs::File, io::Read, path::Path};

use hashbrown::HashMap;

use crate::encoding;

use super::container::Container;

fn parse_database(data: String) -> Vec<String> {
    let mut lines = Vec::new();
    let mut block = String::new();
    let mut in_block = false;

    for line in data.lines() {
        if line.starts_with("@CONTAINER") && !in_block {
            in_block = true;
            block = String::new();
        } else if line.starts_with("@CONTAINER") && in_block {
            panic!("Invalid database file");
        } else if line.starts_with("@END") && in_block {
            in_block = false;
            lines.push(block.clone());
        } else if line.starts_with("@END") && !in_block {
            panic!("Invalid database file");
        } else if in_block {
            block.push_str(line);
            block.push('\n');
        }
    }

    lines
}

fn load_containers(file: String) -> Vec<HashMap<String, Container>> {
    let parsed = {
        let mut file = File::open(file).expect("Unable to open file");
        let data = encoding::decode(file.bytes().map(|x| x.unwrap()).collect());
        parse_database(data)
    };
    let mut containers = Vec::new();

    for parse in parsed {
        containers.push();
    }

    containers
}

pub struct Database {
    pub filename: String,
    pub containers: Vec<HashMap<String, Container>>,
}

impl Database {
    pub(crate) fn new(filename: String) -> Database {
        let containers: Vec<HashMap<String, Container>>;

        if !Path::new(&filename).exists() {
            File::create(&filename).expect("Unable to Database create file");
            containers = Vec::new();
        } else {
            containers = load_containers(filename.clone());
        }

        Database {
            filename,
            containers,
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
