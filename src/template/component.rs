//! Component Template
//! 

use serde::{Deserialize,Serialize};
use tmflib::tmf620::{
    product_specification::ProductSpecification, 
    product_offering::ProductOffering, 
    category::{CategoryRef,Category}
};
use crate::model::component::product::ProductComponent;
use std::convert::Into;

use super::TEMPLATE_CATEGORY;

/// A Component Template defines how to build a component
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct ComponentTemplate {
    pub name : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component : Option<ProductOffering>,
}

impl ComponentTemplate {
    /// Create a new component template
    pub fn new(name : String) -> ComponentTemplate {
        ComponentTemplate { 
            name ,
            component : None,
        }
    }
}

impl Into<ProductOffering> for ComponentTemplate {
    fn into(self) -> ProductOffering {
        self.component.unwrap()
    }
}
