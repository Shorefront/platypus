//! Product Template
//! 
use tmflib::tmf620::product_offering::ProductOffering;

use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize)]
pub struct ProductTemplate {
    offering    : Option<ProductOffering>,
    components  : Option<Vec<super::component::ComponentTemplate>>,
}

impl ProductTemplate {\
    pub fn new(name : String) -> ProductTemplate {
        let offering = ProductOffering::new(name);
        ProductTemplate { offering, components: None }
    }

}