//! Product Template
//! 
use tmflib::tmf620::product_offering::ProductOffering;

use serde::{Deserialize,Serialize};
use log::info;

use super::component::ComponentTemplate;

#[derive(Debug,Deserialize,Serialize)]
pub struct ProductTemplate {
    offering    : Option<ProductOffering>,
    components  : Option<Vec<super::component::ComponentTemplate>>,
}

impl ProductTemplate {
    pub fn new(name : String) -> ProductTemplate {
        let offering = ProductOffering::new(name);
        ProductTemplate { 
            offering    : Some(offering), 
            components  : None }
    }
    pub fn add_components(mut self, components : &mut Vec<ComponentTemplate>) -> Result<String,String> {
        match self.components {
            Some(mut c) => {
                info!("We have components");
                c.append(components);
            },
            None => {
                self.components = Some(vec![]);
                self.components.unwrap().append(components);
                info!("Created new components[]");
            },
        }
        
        Ok(String::from("Ok"))
    }

}