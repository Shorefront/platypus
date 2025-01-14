//! Configuration Module
//! 


const PLATYPUS_PORT : &str = "8001";
const PLATYPUS_WORKERS : u16 = 4;


#[derive(Clone, Debug, Default)]
pub struct Config {}

impl Config {
    pub fn new() -> Config {
        Config {}
    }
    pub fn get(&self, item: &str) -> Option<String> {
        match item {

            _ => Config::get_default(item),
        }
    }
    pub fn get_default(item : &str) -> Option<String> {
        match item {
            "PLATYPUS_PORT" => Some(PLATYPUS_PORT.to_string()),
            "PLATYPUS_WORKERS" => Some(PLATYPUS_WORKERS.to_string()),
            _ => None,
        }
    }
}