//! Metrics module



use actix_web::{web,HttpResponse};
// use actix_web_prom::PrometheusMetricsBuilder;

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn config_metrics(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    

    cfg
        // .wrap(prom.clone())
        // .service(prom.clone())
        .service(web::resource("/health").to(health));
}