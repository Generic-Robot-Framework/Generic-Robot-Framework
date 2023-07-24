use std::net::TcpStream;
use std::io::Write;

#[derive(Debug)]
pub struct Topic {
    pub name: String,
    pub message_type: Option<String>,
    pub subscribers: Vec<TcpStream>
}

impl Clone for Topic {
    fn clone(&self) -> Topic {
        let mut subscribers: Vec<TcpStream> = vec![];

        for subscriber in &self.subscribers {
            subscribers.push(subscriber.try_clone().unwrap());
        }

        Topic {
            name: self.name.clone(),
            message_type: self.message_type.clone(),
            subscribers,
        }
    }
}

impl Topic {
    pub fn write_to_subscribers(&mut self, buf: &[u8]) {
        let mut streams_to_remove: Vec<usize> = vec![];

        for (index, mut subscriber) in self.subscribers.iter().enumerate() {
            match subscriber.write_all(buf) {
                Ok(_) => {}
                Err(_) => {
                    streams_to_remove.push(index);
                }
            }
        }

        for stream in streams_to_remove {
            let _ = &self.subscribers.remove(stream);
        }
    }
}