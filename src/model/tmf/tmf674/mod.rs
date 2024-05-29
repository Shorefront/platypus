//! TMF674 Module

use std::sync::Mutex;
use tmf674_geographic_site::TMF674GeographicSiteManagement;
use actix_web::{
    // get,
    // patch,
    // post,
    // delete,
    web, 
    // HttpResponse, 
    // Responder
};

pub mod tmf674_geographic_site;

pub fn config_tmf674(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf674 = TMF674GeographicSiteManagement::new(None);
    cfg
        .app_data(web::Data::new(Mutex::new(tmf674.clone())));
}