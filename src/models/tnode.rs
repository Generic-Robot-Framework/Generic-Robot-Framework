use std::env;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

#[non_exhaustive]
#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub rate: u64,
    pub ok: bool,
}

impl Node {
    pub fn register(name: &str, rate: u64) -> Node {
        let build = env::args().nth(1);

        if let Some(build) = build {
            if build == "build" {
                print!("{}", name);
                exit(0);
            }
        }

        Node {
            name: name.to_string(),
            rate,
            ok: true
        }
    }

    pub fn sleep(&self) {
        sleep(Duration::from_millis(self.rate))
    }
}