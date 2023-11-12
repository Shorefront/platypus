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

#[derive(Default, Deserialize,Serialize)]
pub struct TMFError {
   code: String,
   reason: String,
   #[serde(skip_serializing_if = "Option::is_none")]
   message: Option<String>,
   #[serde(skip_serializing_if = "Option::is_none")]
   status: Option<String>,
   #[serde(skip_serializing_if = "Option::is_none")]
   reference_error: Option<String>,
}

impl TMFError {
   pub fn new(code : &str, reason : &str) -> TMFError {
      TMFError { 
         code: code.to_owned(), 
         reason: reason.to_owned(), 
         ..Default::default()
      }
   }
   pub fn message(mut self, message : String) -> TMFError {
      self.message = Some(message);
      self
   }

   pub fn status(mut self, status : String) -> TMFError {
      self.status = Some(status);
      self
   }

   pub fn reference_error(mut self, reference: String) -> TMFError { 
      self.reference_error = Some(reference);
      self
   }
}

impl From<PlatypusError> for TMFError {
   fn from(value: PlatypusError) -> Self {
       TMFError { 
         code: "PLAT001".into(), 
         reason: value.message, 
         message: None, 
         status: None, 
         reference_error: None 
      }
   }
}