//! Platypus Primary Module

#![warn(missing_docs)]

use log::info;

mod model;
#[cfg(feature = "composable")]
mod template;
mod common;

use actix_web::middleware::Logger;
use actix_web::{get,post,web,App, HttpResponse,HttpServer, Responder};

use model::tmf::tmf620::config_tmf620;
use model::tmf::tmf622::config_tmf622;
use model::tmf::tmf629::config_tmf629;
use model::tmf::tmf632::config_tmf632;
use model::tmf::tmf648::config_tmf648;
use model::tmf::tmf674::config_tmf674;

#[cfg(feature = "tmf648_v4")]
use tmflib::tmf648::quote::Quote;

use std::sync::Mutex;

// SurrealDB
use serde::Deserialize;

// New Persistence struct
use common::persist::Persistence;

// TMFLIB
use common::config::Config;


use tmflib::tmf629::customer::Customer;
use tmflib::tmf629::customer::CUST_STATUS;
use tmflib::HasId;

#[cfg(feature = "composable")]
use crate::model::component::*;
#[cfg(feature = "composable")]
use crate::template::*;

//use crate::template::product::ProductTemplate;
//use crate::model::component::product::ProductComponent;

/// Fields for filtering output
#[derive(Clone, Debug, Deserialize)]
pub struct QueryOptions {
    /// Specific set of fields delimited by comma
    fields : Option<String>,
    limit : Option<u16>,
    offset : Option<u16>,
    /// Filter on name
    name : Option<String>,
    /// JSONPath Filtering
    filter: Option<String>,
}

#[post("/tmflib/tmf629/customer")]
pub async fn tmf629_create_handler(
    body : web::Json<Customer>,
) -> impl Responder {
    let mut data = body.into_inner();
    // Since this a new customer we have to regenerate the id / href
    data.generate_id();
    // Now that we have an id, we can generate a new code.
    data.generate_code(None);
    data.status = Some(CUST_STATUS.to_string());
    HttpResponse::Ok().json(data)
}

#[get("/tmflib/tmf629/customer/{id}")]
pub async fn tmf629_get_handler(

) -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    env_logger::init();

    info!("Starting {pkg} v{ver}");

    // Data objects to be pass in
    let persist = Persistence::new().await;
    let config = Config::new();

    // Extract port crom config, default if not found
    let port = config.get("PLATYPUS_PORT").unwrap_or("8000".to_string());
    let port = port.parse::<u16>().unwrap();
   
    HttpServer::new(move || {
        App::new()
            // Using the new configure() approach, we cannot pass persis in as
            // configure() does not take additional arguments
            .app_data(web::Data::new(Mutex::new(persist.clone())))
            .app_data(web::Data::new(Mutex::new(config.clone())))
            // New simple config functions.
            .configure(config_tmf620)
            .configure(config_tmf622)
            .configure(config_tmf629)
            .configure(config_tmf648)
            .configure(config_tmf632)
            .configure(config_tmf674)
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0",port))?
        .run()
        .await
}
