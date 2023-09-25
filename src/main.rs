
use log::info;

mod model;
mod template;
mod common;

use common::config::Config;
use tmflib::tmf620::product_specification::{ProductSpecification, ProductSpecificationCharacteristic};
use tmflib::tmf620::tmf620_catalog_management::TMF620CatalogueManagement;

use crate::template::component::ComponentTemplate;
//use crate::template::product::ProductTemplate;
use crate::model::component::product::ProductComponent;

#[warn(missing_docs)]

fn main() {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    env_logger::init();

    info!("Starting {pkg} v{ver}");

    let _cfg = Config::new();
    let char1 = ProductSpecificationCharacteristic::new(String::from("Bandwidth"))
        .cardinality(1, 1)
        .description(String::from("Mandatory attribute for Access"));
    let spec = ProductSpecification::new(String::from("AccessSpecification"))
        .with_charateristic(char1);
    
    //dbg!(&spec);
    // Create a component template for our product template
    let comp_template = ComponentTemplate::new(String::from("Access"))
        .with_specification(spec);
    dbg!(&comp_template);
    // Create Component from template
    let component = ProductComponent::from(comp_template);
    dbg!(&component);
    
    let _tmf620 = TMF620CatalogueManagement::new();
    //tmf620.add_offer(prod_template);
    // Convert template into offer for storage
    //dbg!(prod_template);
}
