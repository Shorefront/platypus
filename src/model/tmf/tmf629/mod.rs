//! TMF629 Module

use std::sync::Mutex;
use actix_web::{
    // get,
    // post,
    web, 
    // HttpResponse, 
    // Responder
};

// TMFLIB

// use crate::common::error::PlatypusError;
// use crate::common::persist::Persistence;
// use crate::QueryOptions;

pub mod tmf629_customer_management;
use tmf629_customer_management::TMF629CustomerManagement;

pub fn config_tmf629(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf629 = TMF629CustomerManagement::new();
    cfg
        .app_data(web::Data::new(Mutex::new(tmf629)));
}