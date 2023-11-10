//! Component Template
//! 

use serde::{Deserialize,Serialize};
use tmflib::tmf620::product_offering::ProductOffering;
use std::convert::Into;

/// A Component Template defines how to build a component
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct ComponentTemplate {
    pub name : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component : Option<ProductOffering>,
}

impl Into<ProductOffering> for ComponentTemplate {
    fn into(self) -> ProductOffering {
        self.component.unwrap()
    }
}
