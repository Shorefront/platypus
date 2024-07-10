//! TMF648 Module
//! 
//! use std::sync::Mutex;

use std::sync::Mutex;
use actix_web::{
    // get,
    post,
    web, 
    HttpResponse, 
    Responder,
};

// TMFLIB
use tmflib::tmf648::quote::Quote;

// use crate::common::error::PlatypusError;
// use crate::common::persist::Persistence;
// use crate::QueryOptions;

pub mod tmf648_quote_management;
use tmf648_quote_management::TMF648QuoteManagement;

#[post("/tmflib/tmf648/quote")]
pub async fn tmf648_create_handler(
    body : web::Json<Quote>
) -> impl Responder {
    let data = body.into_inner();
    HttpResponse::Ok().json(data)
}

pub fn config_tmf648(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf648 = TMF648QuoteManagement::new();
    cfg
        .service(tmf648_create_handler)
        .app_data(web::Data::new(Mutex::new(tmf648)));
}