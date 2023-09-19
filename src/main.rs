
use log::info;

mod model;
mod template;
mod common;

use common::config::Config;

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
    let _result = template.add_components(&mut vec![component1,component2]);
    //let _result = template.add_component(component2);
    
}