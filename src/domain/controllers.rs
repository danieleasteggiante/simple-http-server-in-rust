use std::fs::File;
use std::io::Write;
use serde::de::Unexpected::Str;
use crate::domain::http_request::HttpRequest;
use crate::domain::http_response::HttpResponse;

pub fn get_echo(request: &HttpRequest) -> HttpResponse {
    let parts: Vec<&str> = request.target.split('/').collect();
    let body_content = parts[parts.len() - 1].to_string();
    HttpResponse::new(
        200,
        "OK".to_string(),
        vec![
            ("Content-Type".to_string(), "text/plain\r\n".to_string()),
            ("Content-Length".to_string(), (body_content.len().to_string() + "\r\n").to_string()),
        ],
        body_content
    )
}
pub fn user_agent(request: &HttpRequest) -> HttpResponse {
    let error = "Error: User-Agent not found".to_string();
    let user_agent = request.headers.get("User-Agent").unwrap_or(&error);
    HttpResponse::new(
        200,
        "OK".to_string(),
        vec![
            ("Content-Type".to_string(), "text/plain\r\n".to_string()),
            ("Content-Length".to_string(), (user_agent.len().to_string() + "\r\n").to_string()),
        ],
        user_agent.to_string()
    )
}

pub fn get_file(request: &HttpRequest) -> HttpResponse {
    let parts: Vec<&str> = request.target.split('/').collect();
    let file_name = parts[parts.len() - 1].to_string();
    let file_path = format!("{}/{}", request.files_directory.as_ref().expect("Files directory not present"), file_name);
    println!("File path: {}", file_path);
    let file_content = std::fs::read_to_string(file_path);
    match file_content {
        Ok(content) => HttpResponse::new(200, "OK".to_string(),
            vec![
                ("Content-Type".to_string(), "application/octet-stream\r\n".to_string()),
                ("Content-Length".to_string(), (content.len().to_string() + "\r\n").to_string()),
            ], content),
        Err(_) =>  HttpResponse::new(404, "Not Found".to_string(), vec![], "File not found".to_string()),
    }
}
pub fn post_file(request: &HttpRequest) -> HttpResponse {
    let parts: Vec<&str> = request.target.split('/').collect();
    let file_name = parts[parts.len() - 1].to_string();
    let file_path = format!("{}/{}", request.files_directory.as_ref().expect("Files directory not present"), file_name);

    match write_file_from_post_request(request, file_path) {
        Ok(content) => HttpResponse::new(201, "Created".to_string(),vec![], String::new()),
        Err(_) =>  HttpResponse::new(404, "Not Found".to_string(), vec![], "File not found".to_string()),
    }
}

fn write_file_from_post_request(request: &HttpRequest, file_path: String) -> std::io::Result<()> {
    let mut file = File::create(file_path).expect("Error while creating file");
    file.write_all(request.body.as_ref().unwrap().as_bytes()).expect("Error while writing to file");
    Ok(())
}