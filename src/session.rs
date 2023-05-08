use std::sync::Mutex;

pub struct Session {
    id: u64,
    sequence: Mutex<u64>,
}

impl Session {
    pub fn new(id: u64) -> Session {
        Session { id, sequence: Mutex::new(1) }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn next_sequence(&self) -> u64 {
        let mut seq = self.sequence.lock().unwrap();
        *seq += 1;
        *seq
    }
}
