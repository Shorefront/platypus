//! Product Template
//! 
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::category::{Category,CategoryRef};
use tmflib::tmf620::bundled_product_offering::BundledProductOffering;

use serde::{Deserialize,Serialize};
use std::convert::Into;

use super::component::ComponentTemplate;
use super::TEMPLATE_CATEGORY;

#[derive(Clone, Debug,Deserialize,Serialize)]
pub struct ProductTemplate {
    pub name        : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering    : Option<ProductOffering>,
    pub components  : Vec<super::component::ComponentTemplate>,
}

impl ProductTemplate {
    pub fn new(name : String) -> ProductTemplate {
        let offering = ProductOffering::new(name.clone());
        let cat_ref = Category::new(TEMPLATE_CATEGORY.to_string());
        let mut offering = offering.with_category(CategoryRef::from(&cat_ref));
        // All ProductTemplate are bundles
        offering.is_bundle = Some(true);
        offering.bundled_product_offering = Some(vec![]);
        ProductTemplate { 
            name        : name.clone(),
            offering    : Some(offering), 
            components  : vec![]
        }
    }
  
    pub fn with_component(mut self, components : ComponentTemplate) -> ProductTemplate {
        // Components are represented as bundled offers within the parent offering also
        // This means we also need to update the offering to include the BundledProductOffer
        let po : ProductOffering = components.clone().into();
        // Need to convert from ProductOffering into BundledProductOffering before adding to bundle
        self.offering.as_mut().unwrap().bundled_product_offering.as_mut().unwrap().push(BundledProductOffering::from(po));
        // Also add to list of components
        self.components.push(components);
        self
    }

    /// Generate a new product based off a product template, converting each component template into a component
    pub fn instantiate(self) -> ProductOffering {
        // Step1: Create ProductOffering from template
        // Step2: Instantiate each component template into a component
        // Does that even make sense? Surely we match components from the existing catalogue first?
        let mut offering : ProductOffering = self.clone().into();
        self.components.iter().for_each(|c| {
            let po : ProductOffering = c.clone().into();
            let bundle = BundledProductOffering::from(po);
            offering.bundled_product_offering.as_mut().unwrap().push(bundle);
        });
        offering 
    }

}

impl Into<ProductOffering> for ProductTemplate {
    fn into(self) -> ProductOffering {
        // Use clone here to ensure originating template is unchanged
        self.offering.unwrap().clone()
    }
}
