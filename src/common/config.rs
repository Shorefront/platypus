//! Configuration Module
//! 

use std::env;

const PLATYPUS_PORT : &str = "8001";
const PLATYPUS_WORKERS : u16 = 4;
const DB_HOST : &str = "wss://platypus-06a3rhk0qlrtj092qq5dgtl91o.aws-use1.surreal.cloud";
const DB_NS : &str = "tmf";
const DB_USER : &str = "platypus";
const DB_PASS : &str = "Platypus2025!";


#[derive(Clone, Debug, Default)]
pub struct Config {}

impl Config {
    pub fn new() -> Config {
        Config {}
    }
    pub fn get(&self, item: &str) -> Option<String> {
        match env::var(item) {
            Ok(e) => Some(e),
            _ => Config::get_default(item),
        }
    }
    pub fn get_default(item : &str) -> Option<String> {
        match item {
            "PLATYPUS_PORT" => Some(PLATYPUS_PORT.to_string()),
            "PLATYPUS_WORKERS" => Some(PLATYPUS_WORKERS.to_string()),
            "DB_HOST" => Some(DB_HOST.to_string()),
            "DB_NS" => Some(DB_NS.to_string()),
            "DB_USER" => Some(DB_USER.to_string()),
            "DB_PASS" => Some(DB_PASS.to_string()),
            _ => None,
        }
    }
}