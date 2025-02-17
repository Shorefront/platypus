//! Platypus Primary Module

#![warn(missing_docs)]

use actix_web::dev::Extensions;
use log::{debug,info};

mod model;
#[cfg(feature = "composable")]
mod template;
mod common;

use std::any::Any;
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;

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
#[cfg(feature = "tmf645")]
use model::tmf::tmf645::config_tmf645;
#[cfg(feature = "tmf648")]
use model::tmf::tmf648::config_tmf648;
#[cfg(feature = "tmf674")]
use model::tmf::tmf674::config_tmf674;

#[cfg(feature = "metrics")]
use common::metrics::config_metrics;
#[cfg(feature = "metrics")]
use actix_web_prom::PrometheusMetricsBuilder;

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
        debug!("New Connection: {} {} {}",bind.to_string(),peer.to_string(),ttl.unwrap_or_default());
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    env_logger::init();

    info!("Starting {pkg} v{ver}");

    let config = Config::new();


    
    info!("Connected.");

    // Extract port crom config, default if not found

    let port = config.get("PLATYPUS_PORT").unwrap_or("8001".to_string());
    let port = port.parse::<u16>().unwrap();

    info!("Listening on port {port}.");

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Could not install AWS LC provider");

    // Data objects to be pass in
    info!("Connecting to SurrealDB...");
    let persist = Persistence::new(&config).await;
        // let persis = Persistence::default();

    let cert_file = config.get("TLS_CERT").unwrap_or("certs/cert.pem".to_string());
    let key_file = config.get("TLS_KEY").unwrap_or("certs/key.pem".to_string());

    let mut certs_file = BufReader::new(File::open(cert_file).expect("TLS: Could not open cert.pem"));
    let mut key_file = BufReader::new(File::open(key_file).expect("TLS: Could not open key.pem"));

    let tls_certs = rustls_pemfile::certs(&mut certs_file)
        .collect::<Result<Vec<_>, _>>()
        .expect("TLS: Could not process certificates");

    let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file)
        .next()
        .expect("TLS: Could not process private key")
        .expect("TLS: No private key found");

    let tls_config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
        .expect("TLS: Could not create TLS configuration");
   
    let mut labels = HashMap::new();
    labels.insert("Application".to_string(), "Platypus".to_string());
    let prom = actix_web_prom::PrometheusMetricsBuilder::new("api")
        .endpoint("metrics")
        .const_labels(labels)
        .build()
        .unwrap();
   
    HttpServer::new(move || {
        debug!("Creating new server instance...");
 
        let mut app = App::new()
                .app_data(web::Data::new(Mutex::new(persist.clone())))
                .app_data(web::Data::new(Mutex::new(config.clone())))
                .wrap(prom.clone())
                .wrap(Logger::default());

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

            #[cfg(feature = "tmf645")] 
            {
                debug!("Adding module: TMF645");
                app = app.configure(config_tmf645);
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

            #[cfg(feature = "metrics")]
            {
                debug!("Adding module: Metrics");
                app = app.configure(config_metrics);
            }
            app
    })
        .on_connect(log_conn_info)
        // .bind(("0.0.0.0",port))?
        .bind_rustls_0_23(("0.0.0.0",port),tls_config)?
        .run()
        .await
}
