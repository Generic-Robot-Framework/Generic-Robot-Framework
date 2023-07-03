use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub rate: u64,
    pub ok: bool
}

impl Node {
    pub fn new(name: String, rate: u64) -> Node {
        Node {
            name,
            rate,
            ok: true
        }
    }

    pub fn sleep(&self) {
        sleep(Duration::from_millis(self.rate))
    }
}