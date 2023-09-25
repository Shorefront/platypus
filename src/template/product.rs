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
        let mut offering = offering.with_category(CategoryRef::from(&cat_ref));
        offering.bundled_product_offering = Some(vec![]);
        ProductTemplate { 
            offering    : Some(offering), 
            components  : vec![]
        }
    }
  
    pub fn with_component(mut self, components : ComponentTemplate) -> ProductTemplate {
        // Components are represented as bundled offers within the parent offering also
        // This means we also need to update the offering to include the BundledProductOffer
        let po : ProductOffering = components.clone().into();
        self.offering.as_mut().unwrap().bundled_product_offering.as_mut().unwrap().push(po);
        // Also add to list of components
        self.components.push(components);
        self
    }

}

impl Into<ProductOffering> for ProductTemplate {
    fn into(self) -> ProductOffering {
        self.offering.unwrap()
    }
}
