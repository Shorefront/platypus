//! Component Template
//! 

use serde::{Deserialize,Serialize};
use tmflib::tmf620::{product_specification::ProductSpecification, product_offering::ProductOffering};

/// A Component Template defines how to build a component
#[derive(Debug,Deserialize,Serialize)]
pub struct ComponentTemplate {
    pub name : String,
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

    /// Add specification to this component
    pub fn with_specification(mut self, specification : ProductSpecification) -> ComponentTemplate {
        self.component = Some(ProductOffering::new(self.name.clone())
            .with_specification(specification));
        self
    }
}