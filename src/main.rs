
use log::info;

mod model;
mod template;
mod common;

use common::config::Config;
use tmflib::tmf620::product_specification::{ProductSpecification, ProductSpecificationCharacteristic};
use tmflib::tmf620::tmf620_catalog_management::TMF620CatalogueManagement;

use crate::template::component::ComponentTemplate;
//use crate::template::product::ProductTemplate;
//use crate::model::component::product::ProductComponent;
use crate::template::product::ProductTemplate;

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
    let spec1 = ProductSpecification::new(String::from("AccessSpecification"))
        .with_charateristic(char1);

    let char2 = ProductSpecificationCharacteristic::new(String::from("RoutingProtocol"))
        .description(String::from("Which routing protocol to use"));
    let spec2 = ProductSpecification::new(String::from("ServiceSpecification"))
        .with_charateristic(char2);
    
    // Create a component template for our product template
    let comp_template1 = ComponentTemplate::new(String::from("Access"))
        .with_specification(spec1);
    let comp_template2 = ComponentTemplate::new(String::from("Service"))
        .with_specification(spec2);

    // Create a product template and add in required and optional components
    let prod_template = ProductTemplate::new(String::from("FixedProductTemplate"))
        .with_component(comp_template1)
        .with_component(comp_template2);
    
    dbg!(&prod_template);
    
    let _tmf620 = TMF620CatalogueManagement::new();
    //tmf620.add_offer(prod_template);
    // Convert template into offer for storage
    //dbg!(prod_template);
}
