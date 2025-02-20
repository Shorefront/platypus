//! Hub module for end-point registration

use actix_web::{post,delete,web, HttpResponse, Responder};
use super::error::PlatypusError;
use crate::common::persist::Persistence;
use std::sync::Mutex;
use log::error;
use serde::Serialize;

pub fn render_register_hub<T : Serialize>(output : Result<T,PlatypusError>) -> HttpResponse {
    match output {
        Ok(_b) => HttpResponse::NoContent().finish(),
        Err(e) => {
            error!("Could not delete: {e}");
            HttpResponse::BadRequest().json(e)
        },     
    }     
}

pub fn render_delete_hub<T : Serialize>(output : Result<T,PlatypusError>) -> HttpResponse {
    match output {
        Ok(_b) => HttpResponse::NoContent().finish(),
        Err(e) => {
            error!("Could not delete: {e}");
            HttpResponse::BadRequest().json(e)
        },     
    }     
}

#[post("/tmf-api/hub")]
pub async fn hub_handle_post(
    path : web::Path<String>,
    raw: web::Bytes,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    HttpResponse::Ok().body("Create hub!")
}

#[delete("/tmf-api/hub/{hub_id}")]
pub async fn hub_handle_delete(
    path : web::Path<String>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    HttpResponse::Ok().body("Delete hub!")
}

pub fn config_hub(cfg: &mut web::ServiceConfig) {
    cfg.service(hub_handle_post);
    cfg.service(hub_handle_delete);
}