//! Metrics module

use std::collections::HashMap;

use actix_web::{web,HttpResponse};
use actix_web_prom::{PrometheusMetrics,PrometheusMetricsBuilder};

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn config_metrics(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let mut labels = HashMap::new();
    labels.insert("Application".to_string(),"Platypus".to_string());
    let prom = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();
    cfg
        // .wrap(prom.clone())
        // .service(prom.clone())
        .service(web::resource("/health").to(health));
}