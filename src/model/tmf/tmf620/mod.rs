//! TMF620 Module
//! 
use actix_web::web;
use std::sync::Mutex;
use tmf620_catalog_management::TMF620CatalogManagement;

pub mod tmf620_catalog_management;

// Place actix_web config functions here



pub fn config_tmf620(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    let tmf620 = TMF620CatalogManagement::new(None);
    cfg.app_data(web::Data::new(Mutex::new(tmf620.clone())));
}