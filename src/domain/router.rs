use crate::domain::http_response::HttpResponse;
use crate::domain::controllers;
use regex::Regex;
use crate::domain::http_request::HttpRequest;

pub struct Route {
    pub verb : String,
    pub target: Regex,
    pub(crate) method: fn(&HttpRequest) -> HttpResponse,
}

impl Route {
    fn new(verb: String, target: &str, method: fn(&HttpRequest) -> HttpResponse) -> Self {
        Route {
            verb,
            target: Regex::new(target).unwrap(),
            method,
        }
    }

    pub(crate) fn matches(&self, request: &str) -> bool {
        self.target.is_match(request)
    }
}

pub fn get_routes() -> Vec<Route> {
    vec![
        Route::new("GET".to_string(),r"^/$", |request| {
            HttpResponse::new(
                200,
                "OK".to_string(),
                vec![("Content-Type".to_string(), "text/plain\r\n".to_string())],
                "Welcome to the home page!".to_string(), )
        }),
        Route::new("GET".to_string(),r"^/echo/([^/]+)$", move |x| controllers::get_echo(x)),
        Route::new("GET".to_string(),r"^/user-agent$", move |x| controllers::user_agent(x)),
        Route::new("GET".to_string(),r"^/files/([^/]+)$", move |x| controllers::get_file(x)),
        Route::new("POST".to_string(),r"^/files/([^/]+)$", move |x| controllers::post_file(x)),
    ]
}