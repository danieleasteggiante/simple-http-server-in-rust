use crate::domain::http_request::HttpRequest;
use crate::domain::http_response::HttpResponse;
use crate::domain::router::Route;

#[derive(Debug)]
pub struct Context {
    pub address: String,
    pub routes: Vec<Route>,
    pub files_directory: String,
}

impl Context {
    pub fn new(address: String, routes: Vec<Route>, files_directory: String) -> Self {
        Context{
            address,
            routes,
            files_directory
        }
    }

    pub fn handle_request(&self, request: &HttpRequest) -> HttpResponse {
        for route in &self.routes {
            if route.matches(&request.target) {
                return (route.method)(&self, &request);
            }
        }
        HttpResponse::new(404, "Not Found".to_string(), vec![], "Route not found".to_string())
    }
}