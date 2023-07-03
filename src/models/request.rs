use std::io::{BufRead, BufReader, Read};
use std::net::{TcpStream};
use crate::models::message::Message;

pub const OK_HTTP_STATUS: &str = "HTTP/1.1 200 OK";

/// Returns a single request and returns it a Vector of Strings
pub fn single_request_to_string_vec(stream: &mut TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(stream);
    buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect()
}

/// Returns a single request and returns it a String
pub fn single_request_to_string(stream: &mut TcpStream) -> String {
    let mut buf = vec![0u8; 1024];
    let mut handle = stream.try_clone().unwrap().take(1024);
    handle.read(&mut buf).unwrap();

    String::from_utf8_lossy(&buf)
        .chars()
        .take_while(|&ch| ch != '\0')
        .collect::<String>()
}

/// Acknowledgement HTTP request
pub fn acknowledgement_http_request() -> String {
    return OK_HTTP_STATUS.to_string()
}

/// Formatting a String to a HTTP request
pub fn string_to_http_request(data: String) -> String {
    let length = data.len();

    format!("{OK_HTTP_STATUS}\r\nContent-Length: {length}\r\nContent: {data}")
}

/// Formatting a &Message to a HTTP request
pub fn message_to_http_request<T: crate::traits::MessageTrait>(data: &Message<T>) -> String {
    let contents = serde_json::to_string(data).unwrap();
    let length = contents.len();

    format!("{OK_HTTP_STATUS}\r\nContent-Length: {length}\r\nContent: {contents}")
}