use crate::domain::http_response::HttpResponse;
use crate::domain::controllers;
use regex::Regex;
use crate::core::context::Context;
use crate::domain::http_request::HttpRequest;
#[derive(Debug)]
pub struct Route {
    pub verb : String,
    pub target: Regex,
    pub(crate) method: fn(&Context, &HttpRequest) -> HttpResponse,
}

impl Route {
    fn new(verb: String, target: &str, method: fn(&Context, &HttpRequest) -> HttpResponse) -> Self {
        Route {
            verb,
            target: Regex::new(target).unwrap(),
            method,
        }
    }

    pub fn default() -> Self {
        Route {
            verb: String::new(),
            target: Regex::new(r"^/$").unwrap(),
            method: |ctx:&Context, req: &HttpRequest| HttpResponse::new(200, "OK".to_string(), vec![], "Default route".to_string()),
        }
    }

    pub fn verb(&mut self, verb: String) -> &mut Self {
        self.verb = verb;
        self
    }

    pub fn target(&mut self, target: String) -> &mut Self {
        self.target = Regex::new(&target).expect("Invalid target regex");
        self
    }

    pub fn method(&mut self, method: fn(&Context, &HttpRequest) -> HttpResponse) -> &mut Self {
        self.method = method;
        self
    }

    pub fn build(&self) -> Route {
        Route {
            verb: self.verb.clone(),
            target: self.target.clone(),
            method: self.method,
        }
    }

    pub fn matches(&self, request: &str) -> bool {
        self.target.is_match(request)
    }
}
pub fn get_example_routes() -> Vec<Route> {
    vec![
        Route::new("GET".to_string(),r"^/$", |ctx,req| {
            HttpResponse::new(
                200,
                "OK".to_string(),
                vec![("Content-Type".to_string(), "text/plain\r\n".to_string())],
                "Welcome to the home page!".to_string(), )
        }),
        Route::new("GET".to_string(),r"^/echo/([^/]+)$", move |ctx,req| controllers::get_echo(ctx, req)),
        Route::new("GET".to_string(),r"^/user-agent$", move |ctx,req| controllers::user_agent(ctx, req)),
        Route::new("GET".to_string(),r"^/files/([^/]+)$", move |ctx,req| controllers::get_file(ctx, req)),
        Route::new("POST".to_string(),r"^/files/([^/]+)$", move |ctx,req| controllers::post_file(ctx, req)),
    ]
}