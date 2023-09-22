
use log::info;

mod model;
mod template;
mod common;

use common::config::Config;
use tmflib::tmf620::category::{Category,CategoryRef};
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::product_offering::ProductOffering;

use crate::template::{product::ProductTemplate, component::ComponentTemplate};

#[warn(missing_docs)]

fn main() {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    env_logger::init();

    info!("Starting {pkg} v{ver}");

    let _cfg = Config::new();

    // We wish to create a template
    let prod_template = ProductTemplate::new(String::from("MyTemplate"));
    // Then we wish to add this template to our catalogue
    let catalog = Catalog::new();
    // Convert template into offer for storage
    let po : ProductOffering = prod_template.into();
    //let _result = catalog.add_po(po);
    dbg!(catalog);
}