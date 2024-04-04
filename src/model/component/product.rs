//! Product Component
//! 
use serde::{Deserialize,Serialize};
use tmflib::tmf620::category::{Category,CategoryRef};
use tmflib::tmf620::product_offering::ProductOffering;

use crate::template::component::ComponentTemplate;

use std::convert::Into;

use super::COMPONENT_CATEGORY;


/// Product Component
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct ProductComponent {
    offer   : ProductOffering,   
}

impl ProductComponent {
    pub fn from_offer(offer : ProductOffering) -> ProductComponent {
        ProductComponent {
            offer
        }
    }
}

/// Create a ProductOffering from a ProductComponent for storage
impl Into<ProductOffering> for ProductComponent {

    fn into(self) -> ProductOffering {
        
        // Clone source template so we don't change it
        let mut new_self = self.clone();
        // When converting we need to change the category from 'Template' to 'Component'
        let comp_cat = Category::new(COMPONENT_CATEGORY.to_string());
        new_self.offer.category = Some(vec![CategoryRef::from(&comp_cat)]);
        new_self.offer
    }
}


// Instantiate a Product Component from a Product Template
// A component template contains an offer 
impl From<ComponentTemplate> for ProductComponent {
    fn from(ct : ComponentTemplate) -> ProductComponent {
         // Clone source template so we don't change it
         let mut pc = ProductComponent::from_offer(ct.component.clone().unwrap());
         let comp_cat = Category::new(COMPONENT_CATEGORY.to_string());

         // We need to create a link from ComponentTemplate to ProductComponent
         // We will do this using ProductOfferRelationship
        let remote_po : ProductOffering = ct.into();

         pc.offer.link_po(remote_po, "Template", "template");
         
         pc.offer.category = Some(vec![CategoryRef::from(&comp_cat)]);
         pc
    }
}