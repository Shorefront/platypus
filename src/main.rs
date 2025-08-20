//! Platypus Primary Module

#![warn(missing_docs)]

use actix_web::dev::Extensions;
use log::{debug, error, info};

mod common;
mod model;
#[cfg(feature = "composable")]
mod template;

use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use actix_web::middleware::Logger;
use actix_web::{rt::net::TcpStream, web, App, HttpResponse, HttpServer};

#[cfg(feature = "tmf620")]
use model::tmf::tmf620::config_tmf620;
#[cfg(feature = "tmf622")]
use model::tmf::tmf622::config_tmf622;
#[cfg(feature = "tmf629")]
use model::tmf::tmf629::config_tmf629;
#[cfg(all(feature = "tmf632", feature = "v4"))]
use model::tmf::tmf632::config_tmf632;
#[cfg(all(feature = "tmf632", feature = "v5"))]
use model::tmf::tmf632::config_tmf632_v5;
#[cfg(feature = "tmf633")]
use model::tmf::tmf633::config_tmf633;
#[cfg(feature = "tmf637")]
use model::tmf::tmf637::config_tmf637;
#[cfg(feature = "tmf638")]
use model::tmf::tmf638::config_tmf638;
#[cfg(feature = "tmf639")]
use model::tmf::tmf639::config_tmf639;
#[cfg(feature = "tmf645")]
use model::tmf::tmf645::config_tmf645;
#[cfg(feature = "tmf648")]
use model::tmf::tmf648::config_tmf648;
#[cfg(feature = "tmf663")]
use model::tmf::tmf663::config_tmf663;
#[cfg(feature = "tmf674")]
use model::tmf::tmf674::config_tmf674;

use std::sync::Mutex;

// SurrealDB
use serde::Deserialize;

// New Persistence struct
use common::persist::Persistence;

// TMFLIB
use common::config::Config;

// use common::metrics::health_handler;

/// Fields for filtering output
#[derive(Clone, Default, Debug, Deserialize)]
pub struct QueryOptions {
    /// Specific set of fields delimited by comma
    fields: Option<String>,
    limit: Option<u16>,
    offset: Option<u16>,
    /// Filter on name
    name: Option<String>,
    /// Exapand references
    expand: Option<bool>,
    /// Depth of expansion
    depth: Option<u8>,
}

fn log_conn_info(connection: &dyn Any, _data: &mut Extensions) {
    if let Some(sock) = connection.downcast_ref::<TcpStream>() {
        let bind = sock.local_addr().unwrap();
        let peer = sock.peer_addr().unwrap();
        let ttl = sock.ttl().ok();
        debug!(
            "New Connection: {} {} {}",
            bind.to_string(),
            peer.to_string(),
            ttl.unwrap_or_default()
        );
    }
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
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
    let persist = match Persistence::new(&config).await {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to connect to SurrealDB: {}", e);
            return Ok(());
        }
    };

    let cert_file = config
        .get("TLS_CERT")
        .unwrap_or("certs/cert.pem".to_string());
    let key_file = config.get("TLS_KEY").unwrap_or("certs/key.pem".to_string());

    info!("Using certificate: {} ", cert_file);
    info!("Using key: {} ", key_file);

    let mut certs_file =
        BufReader::new(File::open(cert_file).expect("TLS: Could not open cert.pem"));
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
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();

    HttpServer::new(move || {
        debug!("Creating new server instance...");

        let mut app = App::new()
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
            app = app.configure(config_tmf629);
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

        #[cfg(feature = "tmf637")]
        {
            debug!("Adding module: TMF637");
            app = app.configure(config_tmf637);
        }

        #[cfg(feature = "tmf638")]
        {
            debug!("Adding module: TMF638");
            app = app.configure(config_tmf638);
        }

        #[cfg(feature = "tmf639")]
        {
            debug!("Adding module: TMF639");
            app = app.configure(config_tmf639);
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

        #[cfg(feature = "tmf663")]
        {
            debug!("Adding module: TMF663");
            app = app.configure(config_tmf663);
        }

        #[cfg(feature = "tmf674")]
        {
            debug!("Adding module: TMF674");
            app = app.configure(config_tmf674);
        }
        #[cfg(feature = "events")]
        {
            debug!("Adding module: Events");
            app = app.configure(common::hub::config_hub);
        }

        app.service(web::resource("/health").to(health))
            .wrap(prom.clone())
            .wrap(Logger::default())
    })
    .on_connect(log_conn_info)
    // .bind(("0.0.0.0",port))?
    .bind_rustls_0_23(("0.0.0.0", port), tls_config)?
    .run()
    .await
}
