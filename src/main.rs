mod stores {
    pub mod container;
    pub mod db;
}

pub mod values {
    pub mod key;
}

fn main() {
    let mut db = stores::db::Database::new("test.db".to_string());
    db.create_container("TestContainer".to_string());

    let container = db.get_container("TestContainer");
    if container.is_none() {
        panic!("Unable to get container");
    }
}
