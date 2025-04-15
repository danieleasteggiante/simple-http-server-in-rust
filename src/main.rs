mod domain;
mod core;

use std::io::{BufRead, BufReader, Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::net::TcpStream;
use std::{env, thread};
use std::sync::Arc;
use crate::core::context::Context;
use crate::domain::http_response::HttpResponse;
use crate::core::engine;

fn main() {
    let config_file_path = env::var("CONFIG_FILE").unwrap_or("config.json".to_string());
    let context = Arc::new(engine::Engine::from_config(config_file_path.to_string()).create_context());
    let listener = TcpListener::bind(context.address.clone()).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.expect("Error accepting connection");
        let context_share = Arc::clone(&context); // Clona il riferimento
        thread::spawn(move || handle_request(stream, context_share));
    }
}

fn get_files_directory() -> String {
    let args: Vec<String> = env::args().collect();
    args.iter()
        .position(|a| a == "--directory")
        .and_then(|p| args.get(p + 1))
        .cloned()
        .unwrap_or("/tmp".to_string())
}

fn handle_request(mut stream: TcpStream, context: Arc<Context>) {
    println!("Incoming connection from {:?} - thread {:?}", stream.peer_addr().unwrap(),  thread::current().id());
    let mut buf_reader = BufReader::new(&mut stream);
    let raw_request = get_request(&mut buf_reader);
    let request = domain::http_request::HttpRequest::from_raw(&raw_request).expect("Error parsing request");
    let response = context.handle_request(&request).as_bytes();
    stream.write_all(response.as_slice()).unwrap();
}

fn handle_default_request(mut stream: TcpStream, context: Arc<Context>) {
    println!("Incoming connection from {:?} - thread {:?}", stream.peer_addr().unwrap(),  thread::current().id());
    let mut buf_reader = BufReader::new(&mut stream);
    let request = get_request(&mut buf_reader);
    let response = get_response(&request, context).as_bytes();
    stream.write_all(response.as_slice()).unwrap();
}

fn get_request(buf_reader: &mut BufReader<&mut TcpStream>) -> String {
    let (content_length, mut parts) = handle_parts(buf_reader);
    if content_length > 0 {
        let mut body = vec![0; content_length];
        buf_reader.read_exact(&mut body).expect("Error reading body");
        let body_string = String::from("Body: ") + String::from_utf8_lossy(&body).as_ref();
        parts.push_str(body_string.as_str());
    }
    parts
}

fn handle_parts(buf_reader: &mut BufReader<&mut TcpStream>) -> (usize,String) {
    let mut content_length = 0;
    let mut parts = String::new();
    for line in buf_reader.lines() {
        let line = line.expect("Error reading line");
        if line.is_empty() {
            break;
        }
        if line.starts_with("Content-Length:") {
            let parts: Vec<&str> = line.split(':').collect();
            content_length = parts[1].trim().parse::<usize>().unwrap_or(0);
        }
        parts.push_str(&line);
        parts.push_str("\r\n");
    }
    (content_length, parts)
}

fn get_response(parts: &str, context: Arc<Context>) -> HttpResponse {
    let mut request = domain::http_request::HttpRequest::from_raw(parts).expect("Error parsing request");
    request.files_directory(context.files_directory.to_string());
    domain::router::get_example_routes()
        .iter()
        .filter(|route| route.verb == request.method)
        .find(|route| route.matches(&request.target))
        .map_or_else(
            || HttpResponse::new(404, "Not Found".to_string(),vec![], "Not Found".to_string()),
            |route| (route.method)(&context, &request),
        )
}