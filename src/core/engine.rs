use crate::core::context::Context;
use crate::domain::router::Route;
use crate::domain::{controllers, router};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Engine {
    pub address: String,
    pub routes: Vec<RawRoute>,
    pub files_directory: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawRoute {
    pub method: String,
    pub path: String,
    pub resource: String,
}

impl Engine {
    pub fn from_config(config_file_path: String) -> Engine {
        let dummy_config: serde_json::Value = json!({
            "Address": "127.0.0.1:4221",
            "Routes": [{"Method":"GET", "Path":"^/$", "Resource":"index.html"}],
            "FilesDirectory": "/tmp",
        });
        let config_file = std::fs::read_to_string(config_file_path).or(Err("Error reading config file."));
        if config_file.is_err() {
            let engine : Engine = serde_json::from_value(dummy_config).expect("Error deserializing config.");
            return engine
        }
        serde_json::from_str(&config_file.expect("Error get config file")).expect("JSON was not well-formatted")
    }

    pub fn create_context(&self) -> Context {
        Context {
            address: self.address.clone(),
            routes: self
                .routes
                .iter()
                .clone()
                .map(|r| {
                    Route::default()
                        .verb(r.method.to_string())
                        .target(r.path.to_string())
                        .method(controllers::html)
                        .build()
                })
                .collect(),
            files_directory: self.files_directory.clone(),
        }
    }
}
