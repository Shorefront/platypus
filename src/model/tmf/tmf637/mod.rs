//! TMF637 Product Inventory

use crate::common::error::PlatypusError;
use crate::common::persist::Persistence;
use crate::QueryOptions;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use std::sync::Mutex;

use crate::model::tmf::{
    render_delete_output, render_get_output, render_list_output, render_patch_output,
    render_post_output,
};

use tmflib::tmf637::v4::product::Product;

pub mod tmf637_product_inventory_management;
use tmf637_product_inventory_management::TMF637ProductInventoryManagement;

#[get("/tmf-api/productInventoryManagement/v4/{object}")]
pub async fn tmf637_list_handler(
    path: web::Path<String>,
    query: web::Query<QueryOptions>,
    tmf637: web::Data<Mutex<TMF637ProductInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let object = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf637 = tmf637.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf637.persist(persist.clone());
    match object.as_str() {
        "product" => {
            let products = tmf637.get_products(query_opts).await;
            render_list_output(products)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object")),
    }
}

#[get("/tmf-api/productInventoryManagement/v4/{object}/{id}")]
pub async fn tmf637_get_handler(
    path: web::Path<(String, String)>,
    query: web::Query<QueryOptions>,
    tmf637: web::Data<Mutex<TMF637ProductInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let (object, id) = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf637 = tmf637.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf637.persist(persist.clone());
    match object.as_str() {
        "product" => {
            let product = tmf637.get_product(id, query_opts).await;
            render_get_output(product)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object")),
    }
}

#[post("/tmf-api/productInventoryManagement/v4/{object}")]
pub async fn tmf637_create_handler(
    path: web::Path<String>,
    item: web::Json<Product>,
    tmf637: web::Data<Mutex<TMF637ProductInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let object = path.into_inner();
    let mut tmf637 = tmf637.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf637.persist(persist.clone());
    match object.as_str() {
        "product" => {
            let product = tmf637.add_product(item.into_inner()).await;
            render_post_output(product)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object")),
    }
}

#[patch("/tmf-api/productInventoryManagement/v4/{object}/{id}")]
pub async fn tmf637_patch_handler(
    path: web::Path<(String, String)>,
    item: web::Json<Product>,
    tmf637: web::Data<Mutex<TMF637ProductInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let (object, id) = path.into_inner();
    let mut tmf637 = tmf637.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf637.persist(persist.clone());
    match object.as_str() {
        "product" => {
            let product = tmf637.update_product(id, item.into_inner()).await;
            render_patch_output(product)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object")),
    }
}

#[delete("/tmf-api/productInventoryManagement/v4/{object}/{id}")]
pub async fn tmf637_delete_handler(
    path: web::Path<(String, String)>,
    tmf637: web::Data<Mutex<TMF637ProductInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let (object, id) = path.into_inner();
    let mut tmf637 = tmf637.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf637.persist(persist.clone());
    match object.as_str() {
        "product" => {
            let product = tmf637.delete_product(id).await;
            render_delete_output(product)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object")),
    }
}

pub fn config_tmf637(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf637 = TMF637ProductInventoryManagement::new(None);
    cfg.service(tmf637_list_handler)
        .service(tmf637_get_handler)
        .service(tmf637_create_handler)
        .service(tmf637_patch_handler)
        .service(tmf637_delete_handler)
        .app_data(web::Data::new(Mutex::new(tmf637)));
}
