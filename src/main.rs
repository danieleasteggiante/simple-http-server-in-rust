mod domain;
use std::io::{BufRead, BufReader, Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::net::TcpStream;
use std::{env, thread};
use std::sync::Arc;
use crate::domain::http_response::HttpResponse;

fn main() {
    let files_directory = Arc::new(get_files_directory());
    println!("Files directory: {}", files_directory);
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.expect("Error accepting connection");
        let files_directory = Arc::clone(&files_directory); // Clona il riferimento
        println!("Accepted connection from {:?}", stream.peer_addr().unwrap());
        thread::spawn(move || handle_request(stream, files_directory));
    }
}

fn get_files_directory() -> String {
    let args: Vec<String> = env::args().collect();
    args.iter()
        .position(|a| a == "--directory")
        .and_then(|p| args.get(p + 1))
        .cloned()
        .unwrap_or("/home/daniele/Documenti/APPDEV/codecrafters-http-server/files".to_string())
}

fn handle_request(mut stream: TcpStream, files_directory: Arc<String>) {
    println!("Incoming connection from {:?} - thread {:?}", stream.peer_addr().unwrap(),  thread::current().id());
    let mut buf_reader = BufReader::new(&mut stream);
    let request = get_request(&mut buf_reader);
    let response = get_response(&request, files_directory).as_bytes();
    stream.write_all(response.as_slice()).unwrap();
}

fn get_request(buf_reader: &mut BufReader<&mut TcpStream>) -> String {
    let mut parts = String::new();
    let mut content_length = 0;
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
    if content_length > 0 {
        let mut body = vec![0; content_length];
        buf_reader.read_exact(&mut body).expect("Error reading body");
        let body_string = String::from("Body: ") + String::from_utf8_lossy(&body).as_ref();
        parts.push_str(body_string.as_str());
    }
    parts
}

fn get_response(parts: &str, files_directory: Arc<String>) -> HttpResponse {
    let mut request = domain::http_request::HttpRequest::from_raw(parts).expect("Error parsing request");
    request.files_directory(files_directory.to_string());
    if request.headers.contains_key("Body") {
        request.body = Option::from(request.headers.get("Body").expect("Error body not present").to_string());
        request.headers.remove("Body");
    }
    domain::router::get_routes()
        .iter()
        .filter(|route| route.verb == request.method)
        .find(|route| route.matches(&request.target))
        .map_or_else(
            || HttpResponse::new(404, "Not Found".to_string(),vec![], "Not Found".to_string()),
            |route| (route.method)(&request),
        )
}