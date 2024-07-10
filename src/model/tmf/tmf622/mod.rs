//! TMF622 Module
//! 

use std::sync::Mutex;
pub mod tmf622_product_order_management;

use actix_web::web;
use tmf622_product_order_management::TMF622ProductOrderManagement;

pub fn config_tmf622(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf622 = TMF622ProductOrderManagement::new();
    cfg
        .app_data(web::Data::new(Mutex::new(tmf622.clone())));
}