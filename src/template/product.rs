//! Product Template
//! 
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::category::{Category,CategoryRef};

use serde::{Deserialize,Serialize};
use std::convert::Into;

use super::component::ComponentTemplate;

#[derive(Debug,Deserialize,Serialize)]
pub struct ProductTemplate {
    offering    : Option<ProductOffering>,
    components  : Vec<super::component::ComponentTemplate>,
}

impl ProductTemplate {
    pub fn new(name : String) -> ProductTemplate {
        let offering = ProductOffering::new(name);
        let cat_ref = Category::new(String::from("Templates"));
        let offering = offering.with_category(CategoryRef::from(&cat_ref));
        ProductTemplate { 
            offering    : Some(offering), 
            components  : vec![]
        }
    }
  
    pub fn add_component(&mut self, components : ComponentTemplate) -> Result<String,String> {
        self.components.push(components);
        Ok(String::from("Ok"))
    }

}

impl Into<ProductOffering> for ProductTemplate {
    fn into(self) -> ProductOffering {
        self.offering.unwrap()
    }
}
