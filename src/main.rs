//! Platypus Primary Module

#![warn(missing_docs)]

use actix_web::dev::Extensions;
use log::{debug,info};

mod model;
#[cfg(feature = "composable")]
mod template;
mod common;

use std::any::Any;

use actix_web::middleware::Logger;
use actix_web::{web,App,HttpServer,rt::net::TcpStream};

#[cfg(feature = "tmf620")]
use model::tmf::tmf620::config_tmf620;
#[cfg(feature = "tmf622")]
use model::tmf::tmf622::config_tmf622;
#[cfg(feature = "tmf629")]
use model::tmf::tmf629::config_tmf629;
#[cfg(all(feature = "tmf632",feature = "v4"))]
use model::tmf::tmf632::config_tmf632;
#[cfg(feature = "tmf633")]
use model::tmf::tmf633::config_tmf633;
#[cfg(all(feature = "tmf632",feature = "v5"))]
use model::tmf::tmf632::config_tmf632_v5;
#[cfg(feature = "tmf648")]
use model::tmf::tmf648::config_tmf648;
#[cfg(feature = "tmf674")]
use model::tmf::tmf674::config_tmf674;

use std::sync::Mutex;

// SurrealDB
use serde::Deserialize;

// New Persistence struct
use common::persist::Persistence;

// TMFLIB
use common::config::Config;

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

fn log_conn_info(connection: &dyn Any, _data: &mut Extensions) {
    if let Some(sock) = connection.downcast_ref::<TcpStream>() {
        let bind = sock.local_addr().unwrap();
        let peer = sock.peer_addr().unwrap();
        let ttl = sock.ttl().ok();
        info!("New Connection: {} {} {}",bind.to_string(),peer.to_string(),ttl.unwrap_or_default());
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    env_logger::init();

    info!("Starting {pkg} v{ver}");

    let config = Config::new();

    // Data objects to be pass in
    info!("Connecting to SurrealDB...");
    let persist = Persistence::new(&config).await;
    // let persis = Persistence::default();
    
    info!("Connected.");

    // Extract port crom config, default if not found

    let port = config.get("PLATYPUS_PORT").unwrap_or("8001".to_string());
    let port = port.parse::<u16>().unwrap();

    info!("Listening on port {port}.");
   
    HttpServer::new(move || {
        debug!("Creating new server instance...");
        let mut app = App::new()
            // Using the new configure() approach, we cannot pass persis in as
            // configure() does not take additional arguments
            .app_data(web::Data::new(Mutex::new(persist.clone())))
            .app_data(web::Data::new(Mutex::new(config.clone())));

            // New simple config functions.
            #[cfg(feature = "tmf620")] 
            {
                debug!("Adding module: TMF620");
                app = app.configure(config_tmf620);
            }

            #[cfg(feature = "tmf622")] 
            {
                debug!("Adding module: TMF622");
                app = app.configure(config_tmf622);
            }

            #[cfg(feature = "tmf629")]
            {
                debug!("Adding module: TMF629");
                app = app .configure(config_tmf629);
            }

            #[cfg(feature = "tmf632")] 
            {
                debug!("Adding module: TMF632");
                app = app.configure(config_tmf632);
            }

            #[cfg(feature = "tmf633")]
            {
                debug!("Adding module: TMF633");
                app = app.configure(config_tmf633);
            }

            #[cfg(feature = "tmf648")] 
            {
                debug!("Adding module: TMF648");
                app = app.configure(config_tmf648);
            }
            
            #[cfg(feature = "tmf674")]
            {
                debug!("Adding module: TMF674");
                app =  app.configure(config_tmf674);
            }
        app.wrap(Logger::default())  
    })
        .on_connect(log_conn_info)
        .bind(("0.0.0.0",port))?
        .run()
        .await
}
