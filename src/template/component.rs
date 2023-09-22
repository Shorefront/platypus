//! Component Template
//! 

use serde::{Deserialize,Serialize};

/// A Component Template defines how to build a component
#[derive(Debug,Deserialize,Serialize)]
pub struct ComponentTemplate {
    pub name : String,
}

impl ComponentTemplate {
    /// Create a new component template
    pub fn new(name : String) -> ComponentTemplate {
        ComponentTemplate { 
            name 
        }
    }
}