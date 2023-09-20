
use log::info;

mod model;
mod template;
mod common;

use common::config::Config;
use tmflib::tmf620::category::Category;
use tmflib::tmf620::catalog::Catalog;

use crate::template::{product::ProductTemplate, component::ComponentTemplate};

#[warn(missing_docs)]

fn main() {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    env_logger::init();

    info!("Starting {pkg} v{ver}");

    let _cfg = Config::new();

    let component1 = ComponentTemplate::new(String::from("Access"));
    let component2 = ComponentTemplate::new(String::from("Service"));
    let template = ProductTemplate::new(String::from("Fixed_Internet_Template"));
    let catalog = Catalog::new().name(String::from("Templates"));
    let access_category = Category::new(String::from("Access"));
    let _result = template.add_components(&mut vec![component1,component2]);
    let _result = catalog.add_category(access_category);
    let _result = template.with_category(access_category);
    //let _result = catalog.add(template);
    //let _result = template.add_component(component2);
    
}