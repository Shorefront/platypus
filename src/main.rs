//! Platypus Primary Module

#![warn(missing_docs)]

use log::info;
use log::{debug,info};

mod model;
#[cfg(feature = "composable")]
mod template;
mod common;

use actix_web::middleware::Logger;
use actix_web::{web,App,HttpServer};

#[cfg(feature = "tmf620_v4")]
use model::tmf::tmf620::config_tmf620;
#[cfg(feature = "tmf622_v4")]
use model::tmf::tmf622::config_tmf622;
#[cfg(feature = "tmf629_v4")]
use model::tmf::tmf629::config_tmf629;
#[cfg(feature = "tmf632_v4")]
use model::tmf::tmf632::config_tmf632;
#[cfg(feature = "tmf632_v5")]
use model::tmf::tmf632::config_tmf632_v5;
#[cfg(feature = "tmf648_v4")]
use model::tmf::tmf648::config_tmf648;
#[cfg(feature = "tmf674_v4")]
use model::tmf::tmf674::config_tmf674;
mod metrics;

use metrics::config_metrics;
use actix_web_prom::PrometheusMetricsBuilder;
use std::collections::HashMap;

use std::sync::Mutex;

// SurrealDB
use serde::Deserialize;

// New Persistence struct
use common::persist::Persistence;

// TMFLIB
use common::config::Config;

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
    let port = config.get("PLATYPUS_PORT").unwrap_or("8080".to_string());
    let port = port.parse::<u16>().unwrap();
   
    let mut labels = HashMap::new();
    labels.insert("Application".to_string(),"Platypus".to_string());
    let prom = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();
    
    HttpServer::new(move || {
        let mut app = App::new()
            // Using the new configure() approach, we cannot pass persis in as
            // configure() does not take additional arguments
            .app_data(web::Data::new(Mutex::new(persist.clone())))
            .app_data(web::Data::new(Mutex::new(config.clone())))
            .wrap(prom.clone())
            .wrap(Logger::default());
            // New simple config functions.
            if cfg!(feature = "tmf620_v4") {
                debug!("Adding module: TMF620");
                app = app.configure(config_tmf620);
            }
            if cfg!(feature = "tmf622_v4") {
                debug!("Adding module: TMF622");
                app = app.configure(config_tmf622);
            }
            if cfg!(feature = "tmf629_v4") {
                debug!("Adding module: TMF629");
                app = app .configure(config_tmf629);
            }
            if cfg!(any(feature = "tmf632_v4", feature = "tmf632_v5")) {
                debug!("Adding module: TMF632");
                app = app.configure(config_tmf632);
            }
            if cfg!(feaure = "tmf648_v4") {
                debug!("Adding module: TMF648");
                app = app.configure(config_tmf648);
            }
            if cfg!(feature = "tmf674_v4") {
                debug!("Adding module: TMF674");
                app =  app.configure(config_tmf674);
            }
            
        app
    })
        .bind(("0.0.0.0",port))?
        .run()
        .await
}
