use std::sync::Mutex;

pub struct Session {
    id: Mutex<u64>,
}

impl Session {
    pub fn new(id: u64) -> Session {
        Session { id: Mutex::new(id) }
    }

    pub fn next_id(&self) -> u64 {
        let mut id = self.id.lock().unwrap();
        *id += 1;
        *id
    }
}
