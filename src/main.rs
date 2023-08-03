use std::io::Read;

mod encoding;

mod json {
    mod deserialize;
    mod serialize;
}

mod stores {
    pub mod container;
    pub mod db;
}

pub mod values {
    pub mod key;
}

fn main() {
    let mut file = std::fs::File::open("test.bin").unwrap();
    let decoded = encoding::decode(file.bytes().map(|x| x.unwrap()).collect());
    println!("{}", decoded);
    /*let mut db = stores::db::Database::new("test.db".to_string());
    db.create_container("TestContainer".to_string());

    let container = db.get_container("TestContainer");
    if container.is_none() {
        panic!("Unable to get container");
    }*/
}
