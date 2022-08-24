pub mod entry;

use entry::Entry;

pub struct Todo {
    file_path: String,
    entries: Vec<Entry>,
}

impl Todo {
    pub fn load(path: String) -> Self {
        // TODO Check if existing DB, make new if not
        Self::make_db(&path);
        // TODO Populate entries vec from DB
        Self {
            file_path: path,
            entries: Vec::new(),
        }
    }
    pub fn save(self) {
        // TODO Serialise entries to the DB Json
    }
    fn make_db(path: &String) {
        // Create Empty File in case of no current DB
    }
}
