//! Configuration Module
//! 

#[derive(Clone, Debug, Default)]
pub struct Config {}

impl Config {
    pub fn new() -> Config {
        Config {}
    }
    pub fn get(item: &str) -> Option<String> {
        match item {

            _ => Config::get_default(item),
        }
    }
    pub fn get_default(item : &str) -> Option<String> {
        match item {
            _ => None,
        }
    }
}