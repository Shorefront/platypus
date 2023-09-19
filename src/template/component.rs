//! Component Template
//! 

use serde::{Deserialize,Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct ComponentTemplate {
    pub name : String,
}

impl ComponentTemplate {
    pub fn new(name : String) -> ComponentTemplate {
        ComponentTemplate { name  }
    }
}