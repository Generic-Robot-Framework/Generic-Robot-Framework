use std::fmt::{Debug};
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use serde::de::DeserializeOwned;
use crate::traits::MessageTrait;
use crate::models::message::Message;
use crate::models::request::{message_to_http_request, OK_HTTP_STATUS, single_request_to_string};

#[derive(Debug)]
pub struct Subscriber<T> where T: MessageTrait + DeserializeOwned {
    pub topic_name: String,
    handle: fn(T),
    listener: Arc<Mutex<TcpStream>>,
    content: Option<T>
}

impl<T> Subscriber<T> where T: MessageTrait + DeserializeOwned + 'static {
    pub fn new(topic_name: String, handle: fn(T)) -> Subscriber<T> {
        let topic_name_clone = topic_name.clone();

        let subscriber = Subscriber {
            topic_name,
            handle,
            listener: Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:1312").unwrap())),
            content: None,
        };

        let stream = subscriber.listener.clone();
        std::thread::spawn(move || {
            let data: Message<T> = Message {
                kind: String::from("sub"),
                topic: Some(String::from(topic_name_clone)),
                message: None,
            };

            let request = message_to_http_request(&data);
            stream.lock().unwrap().write_all(request.as_bytes()).ok();
            stream.lock().unwrap().shutdown(Shutdown::Write).ok();

            let response = single_request_to_string(&mut stream.lock().unwrap());

            if response != OK_HTTP_STATUS {
                panic!("Bad response: {}", response)
            }

            loop {
                let response = single_request_to_string(&mut stream.lock().unwrap());
                let data: T = serde_json::from_str(response.as_str()).unwrap();

                if response.len() > 0 {
                    (subscriber.handle)(data);
                }
                else {
                    break;
                }
            }
        });

        return subscriber;
    }
}