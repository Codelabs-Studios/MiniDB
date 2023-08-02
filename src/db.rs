use std::{fs::File, path::Path};

use crate::collection;

pub struct Database {
    filename: String,
    collections: Vec<collection::Collection>,
}

impl Database {
    fn read_collections() -> Vec<collection::Collection> {
        unimplemented!()
    }

    pub fn create_collection() {
        unimplemented!()
    }
    pub fn delete_collection() {
        unimplemented!()
    }

    pub(crate) fn new(filename: String) -> Database {
        if !Path::new(&filename).exists() {
            File::create(&filename).expect("Unable to Database create file");
        }

        Database {
            filename,
            collections: Vec::new(),
        }
    }
}
