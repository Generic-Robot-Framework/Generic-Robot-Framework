use std::fmt::{Debug};
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use serde::de::DeserializeOwned;
use crate::traits::MessageTrait;
use crate::models::message::{get_message_type_as_str, Message};
use crate::models::request::{message_to_http_request, OK_HTTP_STATUS, single_request_to_string};

#[derive(Debug)]
pub struct Subscriber<T, R> where T: MessageTrait + DeserializeOwned {
    pub topic_name: String,
    handle: fn(T, Option<R>),
    listener: Arc<Mutex<TcpStream>>,
    #[allow(dead_code)]
    content: Option<T>
}

impl<T, R> Subscriber<T, R> where T: MessageTrait + DeserializeOwned + 'static, R: Send + 'static + Clone {
    pub fn new(topic_name: &str, handle: fn(T, Option<R>), argument: Option<R>) -> Subscriber<T, R> {
        let topic_name_clone = topic_name.clone().to_string();

        let subscriber = Subscriber {
            topic_name: topic_name.to_string(),
            handle,
            listener: Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:1312").unwrap())),
            content: None,
        };

        let stream = subscriber.listener.clone();

        let data: Message<T> = Message {
            kind: String::from("sub"),
            topic: Some(String::from(topic_name_clone)),
            message: None,
            message_type: get_message_type_as_str::<T>(),
        };

        let request = message_to_http_request(&data);
        stream.lock().unwrap().write_all(request.as_bytes()).ok();
        stream.lock().unwrap().shutdown(Shutdown::Write).ok();

        let response = single_request_to_string(&mut stream.lock().unwrap());

        if response != OK_HTTP_STATUS {
            panic!("Bad response: {}", response)
        }

        std::thread::spawn(move || {
            loop {
                let response = single_request_to_string(&mut stream.lock().unwrap());
                let data: T = serde_json::from_str(response.as_str()).expect("Wrong type provided");

                if response.len() > 0 {
                    if argument.is_some() {
                        (subscriber.handle)(data, Some(argument.clone().unwrap()));
                    }
                    else {
                        (subscriber.handle)(data, None);
                    }
                }
                else {
                    break;
                }
            }
        });

        return subscriber;
    }
}