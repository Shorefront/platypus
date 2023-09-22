
use log::info;

mod model;
mod template;
mod common;

use common::config::Config;
use tmflib::tmf629::customer::Customer;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::tmf620_catalog_management::TMF620CatalogueManagement;

use crate::template::component::ComponentTemplate;
use crate::template::product::ProductTemplate;

#[warn(missing_docs)]

fn main() {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    env_logger::init();

    info!("Starting {pkg} v{ver}");

    let _cfg = Config::new();

    // We wish to create a template
    let mut prod_template = ProductTemplate::new(String::from("MyTemplate"));
    // Create a component template for our product template
    let comp_template = ComponentTemplate::new(String::from("ComponentTemplate"));

    let _result = prod_template.add_component(comp_template);
    // Then we wish to add this template to our catalogue
    let mut tmf620 = TMF620CatalogueManagement::new();
    // Convert template into offer for storage
    let po : ProductOffering = prod_template.into();
    let _result = tmf620.add_offer(po.clone());
    dbg!(po);

    // Create a customer
    let cust = Customer::new(String::from("Shorefront Consulting"));
    dbg!(cust);
}
