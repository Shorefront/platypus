//! Component Template
//! 

use serde::{Deserialize,Serialize};
use tmflib::tmf620::{
    product_specification::ProductSpecification, 
    product_offering::ProductOffering, 
    category::{CategoryRef,Category}
};
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

    /// Add specification to this component
    pub fn with_specification(mut self, specification : ProductSpecification) -> ComponentTemplate {
        let cat = Category::new(TEMPLATE_CATEGORY.to_string());
        // Create a ProductOffering to hold the component specification
        self.component = Some(ProductOffering::new(self.name.clone())
            .with_specification(specification)
            .with_category(CategoryRef::from(&cat))
        );
        self
    }
}

impl Into<ProductOffering> for ComponentTemplate {
    fn into(self) -> ProductOffering {
        self.component.unwrap()
    }
}
