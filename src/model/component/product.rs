//! Product Component
//! 
use serde::{Deserialize,Serialize};
use tmflib::tmf620::product_specification::ProductSpecification;
use tmflib::tmf620::product_offering::ProductOffering;

use std::convert::Into;


/// Product Component
#[derive(Deserialize,Serialize)]
pub struct ProductComponent {
    offer   : ProductOffering,
    specification : ProductSpecification,     
}

impl Into<ProductOffering> for ProductComponent {
    fn into(self) -> ProductOffering {
        self.offer
    }
}