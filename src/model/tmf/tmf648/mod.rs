//! TMF648 Module
//!
//! use std::sync::Mutex;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use std::sync::Mutex;

use crate::model::tmf::{
    render_delete_output, render_get_output, render_list_output, render_patch_output,
    render_post_output,
};

// TMFLIB
use tmflib::tmf648::quote::Quote;

use crate::common::error::PlatypusError;
use crate::common::persist::Persistence;
use crate::QueryOptions;

pub mod tmf648_quote_management;
use tmf648_quote_management::TMF648QuoteManagement;

#[post("/tmf-api/quoteManagement/v4/{object}")]
pub async fn tmf648_create_handler(
    path: web::Path<String>,
    raw: web::Bytes,
    tmf648: web::Data<Mutex<TMF648QuoteManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let object = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf648 = tmf648.lock().unwrap();
    let persist = persist.lock().unwrap();
    // Set persistance into TMF object
    tmf648.persist(persist.clone());
    match object.as_str() {
        "quote" => {
            let quote: Quote =
                serde_json::from_str(json.as_str()).expect("Could not parse TMF648 objet");
            let result = tmf648.add_quote(quote).await;
            render_post_output(result)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object: {object}")),
    }
}

#[patch("/tmf-api/quoteManagement/v4/{object}/{id}")]
pub async fn tmf648_patch_handler(
    path: web::Path<(String, String)>,
    tmf648: web::Data<Mutex<TMF648QuoteManagement>>,
    persist: web::Data<Mutex<Persistence>>,
    raw: web::Bytes,
) -> impl Responder {
    let (object, id) = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf648 = tmf648.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf648.persist(persist.clone());
    match object.as_str() {
        "quote" => {
            let quote: Quote = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf648.update_quote(id, quote).await;
            render_patch_output(result)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("PATCH: Bad object: {object}")),
    }
}

#[get("/tmf-api/quoteManagement/v4/{object}")]
pub async fn tmf648_list_handler(
    path: web::Path<String>,
    query: web::Query<QueryOptions>,
    tmf648: web::Data<Mutex<TMF648QuoteManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let object = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf648 = tmf648.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf648.persist(persist.clone());
    match object.as_str() {
        "quote" => {
            let quotes = tmf648.get_quotes(query_opts).await;
            render_list_output(quotes)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object")),
    }
}

#[get("/tmf-api/quoteManagement/v4/{object}/{id}")]
pub async fn tmf648_get_handler(
    path: web::Path<(String, String)>,
    query: web::Query<QueryOptions>,
    tmf648: web::Data<Mutex<TMF648QuoteManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let (object, id) = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf648 = tmf648.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf648.persist(persist.clone());
    match object.as_str() {
        "quote" => {
            let customers = tmf648.get_quote(id, query_opts).await;
            render_get_output(customers)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object")),
    }
}

#[delete("/tmf-api/geographicSiteManagement/v4/{object}/{id}")]
pub async fn tmf648_delete_handler(
    path: web::Path<(String, String)>,
    tmf648: web::Data<Mutex<TMF648QuoteManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let (object, id) = path.into_inner();
    let mut tmf648 = tmf648.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf648.persist(persist.clone());
    match object.as_str() {
        "quote" => {
            let customers = tmf648.delete_quote(id).await;
            render_delete_output(customers)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object")),
    }
}

pub fn config_tmf648(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf648 = TMF648QuoteManagement::new(None);
    cfg.service(tmf648_get_handler)
        .service(tmf648_list_handler)
        .service(tmf648_create_handler)
        .service(tmf648_patch_handler)
        .service(tmf648_delete_handler)
        .app_data(web::Data::new(Mutex::new(tmf648)));
}
