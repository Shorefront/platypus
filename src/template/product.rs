//! Product Template
//! 
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::category::{Category,CategoryRef};

use serde::{Deserialize,Serialize};
use std::convert::Into;

use super::component::ComponentTemplate;
use super::TEMPLATE_CATEGORY;

#[derive(Debug,Deserialize,Serialize)]
pub struct ProductTemplate {
    pub offering    : Option<ProductOffering>,
    pub components  : Vec<super::component::ComponentTemplate>,
}

impl ProductTemplate {
    pub fn new(name : String) -> ProductTemplate {
        let offering = ProductOffering::new(name);
        let cat_ref = Category::new(TEMPLATE_CATEGORY.to_string());
        let offering = offering.with_category(CategoryRef::from(&cat_ref));
        ProductTemplate { 
            offering    : Some(offering), 
            components  : vec![]
        }
    }
  
    pub fn with_component(mut self, components : ComponentTemplate) -> ProductTemplate {
        self.components.push(components);
        self
    }

}

impl Into<ProductOffering> for ProductTemplate {
    fn into(self) -> ProductOffering {
        self.offering.unwrap()
    }
}
