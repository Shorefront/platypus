//! Error Module
//! 
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize)]
pub struct PlatypusError {
   pub message : String,
}

impl std::fmt::Display for PlatypusError {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f,"Platypus Error: {}",self.message)
   }
}

impl From<surrealdb::Error> for PlatypusError {
   fn from(value: surrealdb::Error) -> Self {
       PlatypusError { message: value.to_string(), }
   }
}

impl From<&str> for PlatypusError {
   fn from(value: &str) -> Self {
       PlatypusError { message: value.to_owned() }
   }
}