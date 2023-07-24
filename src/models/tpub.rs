use std::fmt::{Debug};
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use crate::models::message::Message;
use crate::models::request::{message_to_http_request, OK_HTTP_STATUS, single_request_to_string};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Publisher<T> {
    pub topic_name: String,
    content: Option<T>
}


impl<T> Publisher<T> where T: crate::traits::MessageTrait {
    pub const fn new(topic_name: String) -> Publisher<T> {
        Publisher {
            topic_name,
            content: None,
        }
    }

    pub fn publish(&self, content: T) where T: crate::traits::MessageTrait {
        let mut stream = TcpStream::connect("127.0.0.1:1312").unwrap();

        let data = &Message {
            kind: String::from("pub"),
            topic: Some(self.topic_name.clone()),
            message: Some(content)
        };

        let request = message_to_http_request(data);
        stream.write_all(request.as_bytes()).ok();
        stream.shutdown(Shutdown::Write).ok();

        let response = single_request_to_string(&mut stream);

        if response != OK_HTTP_STATUS {
            panic!("Bad response: {}", response)
        }
    }
}