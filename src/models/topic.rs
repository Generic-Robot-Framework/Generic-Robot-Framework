use std::net::TcpStream;
use std::io::Write;

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
    pub fn write_to_subscribers(&self, buf: &[u8]) {
        for mut subscriber in &self.subscribers {
            subscriber.write_all(buf).unwrap();
        }
    }
}