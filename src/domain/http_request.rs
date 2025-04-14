use std::collections::HashMap;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct HttpRequest {
    pub method: String,
    pub target: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub files_directory: Option<String>,
}

impl HttpRequest {
    pub fn new() -> Self {
        HttpRequest {
            ..Default::default()
        }
    }

    pub fn method(&mut self, method: String) -> &mut Self {
        self.method = method;
        self
    }

    pub fn target(&mut self, target: String) -> &mut Self {
        self.target = target;
        self
    }

    pub fn version(&mut self, version: String) -> &mut Self {
        self.version = version;
        self
    }

    pub fn header(&mut self, key: String, value: String) -> &mut Self {
        self.headers.insert(key, value);
        self
    }

    pub fn headers(&mut self, headers: HashMap<String,String>) -> &mut Self {
        self.headers = headers;
        self
    }

    pub fn body(&mut self, body: String) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn files_directory(&mut self, files_directory: String) -> &mut Self {
        self.files_directory = Some(files_directory);
        self
    }

    pub fn build(&self) -> HttpRequest {
        HttpRequest {
            method: self.method.clone(),
            target: self.target.clone(),
            version: self.version.clone(),
            headers: self.headers.clone(),
            body: self.body.clone(),
            files_directory: self.files_directory.clone(),
        }
    }

    pub fn from_raw(raw: &str) -> Result<Self, Error> {
        let lines = raw.split("\r\n").collect::<Vec<&str>>();
        let request_line = lines[0].split_whitespace().collect::<Vec<&str>>();
        let method = request_line[0].to_string();
        let target = request_line[1].to_string();
        // senza la & lui prova a spostarlo sullo stack ma non sa la dimensione reale
        let headers = HttpRequest::get_headers_if_any(&lines[1..]);
        let body = HttpRequest::get_body_if_any(&lines[1..]);
        Ok(HttpRequest::new()
            .method(method)
            .headers(headers)
            .target(target)
            .body(body)
            .build())
    }

    fn get_headers_if_any(lines: &[&str]) -> HashMap<String, String> {
        if lines.is_empty() {
            return HashMap::new();
        }
        lines.iter().filter(|l| !l.is_empty() || l.starts_with("Body")).map(|l|{
            let header = l.split(": ").collect::<Vec<&str>>();
            (header[0].to_string(), header[1].to_string())
        }).collect()
    }

    fn get_body_if_any(lines: &[&str]) -> String {
        if lines.is_empty() {
            return "".to_string();
        }
        let body = lines.iter()
            .filter(|l| l.starts_with("Body"))
            .map(|l| l.split(": ").nth(1).unwrap_or(""))
            .collect::<Vec<&str>>()
            .join("");

        if body.is_empty() { "".to_string() } else { body }
    }
}
