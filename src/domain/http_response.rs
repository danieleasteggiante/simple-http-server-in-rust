use serde::{Deserialize, Serialize};
use bincode;
use bincode::{Decode, Encode};

#[derive(Encode, Decode, Serialize, Deserialize, Debug)]
pub struct HttpResponse {
    status_code: u16,
    status_reason: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl HttpResponse {
    pub fn new(status_code: u16, status_reason: String, headers: Vec<(String, String)>, body: String) -> Self {
        HttpResponse {
            status_code,
            status_reason,
            headers,
            body,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8>{
       format!(
            "HTTP/1.1 {} {}\r\n{}\r\n{}",
            self.status_code,
            self.status_reason,
            self.headers
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join(""),
            self.body
        ).into_bytes()
    }
}