pub struct Generator {
    prefix: u32,
    last: u64,
    next: u64,
}

impl Generator {
    pub fn new() -> Generator {
        Generator { prefix: 0, last: 0, next: 0 }
    }

    pub fn next(&self) -> u64 {
        0
    }
    fn get_id(&self) -> bool {
        false
    }
}
