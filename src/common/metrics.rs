//! Metrics for the common crate.
//! 

use actix_web::{get,web,HttpResponse,Responder};

#[get("/health")]
async fn health_handler() -> impl Responder {
    HttpResponse::Ok().finish()
}