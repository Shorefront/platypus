//! TMF638 Service Inventory Management

use std::sync::Mutex;
use crate::common::persist::Persistence;
use crate::common::error::PlatypusError;
use crate::QueryOptions;
use actix_web::{
    get,
    patch,
    post,
    delete,
    web, 
    HttpResponse, 
    Responder
};

use crate::model::tmf::{
    render_get_output,
    render_list_output,
    render_post_output,
    render_patch_output,
    render_delete_output,
};

use tmflib::tmf638::service::Service;

pub mod tmf638_service_inventory_management;
use tmf638_service_inventory_management::TMF638ServiceInventoryManagement;


#[get("/tmf-api/serviceInventory/v4/{object}")]
pub async fn tmf638_list_handler(
    path : web::Path<String>,
    query : web::Query<QueryOptions>,
    tmf638: web::Data<Mutex<TMF638ServiceInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let object = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf638 = tmf638.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf638.persist(persist.clone());
    match object.as_str() {
        "service" => {
            let products = tmf638.get_services(query_opts).await;
            render_list_output(products)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

#[get("/tmf-api/serviceInventory/v4/{object}/{id}")]
pub async fn tmf638_get_handler(
    path : web::Path<(String,String)>,
    query : web::Query<QueryOptions>,
    tmf638: web::Data<Mutex<TMF638ServiceInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object, id) = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf638 = tmf638.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf638.persist(persist.clone());
    match object.as_str() {
        "service" => {
            let product = tmf638.get_service(id, query_opts).await;
            render_get_output(product)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}   

#[post("/tmf-api/serviceInventory/v4/{object}")]
pub async fn tmf638_create_handler(
    path : web::Path<String>,
    item: web::Json<Service>,
    tmf638: web::Data<Mutex<TMF638ServiceInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let object = path.into_inner();
    let mut tmf638 = tmf638.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf638.persist(persist.clone());
    match object.as_str() {
        "service" => {
            let product = tmf638.add_service(item.into_inner()).await;
            render_post_output(product)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}   

#[patch("/tmf-api/serviceInventory/v4/{object}/{id}")]
pub async fn tmf638_patch_handler(
    path : web::Path<(String,String)>,
    item: web::Json<Service>,
    tmf637: web::Data<Mutex<TMF638ServiceInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object, id) = path.into_inner();
    let mut tmf637 = tmf637.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf637.persist(persist.clone());
    match object.as_str() {
        "service" => {
            let product = tmf637.update_service(id, item.into_inner()).await;
            render_patch_output(product)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

#[delete("/tmf-api/serviceInventory/v4/{object}/{id}")]
pub async fn tmf638_delete_handler(
    path : web::Path<(String,String)>,
    tmf638: web::Data<Mutex<TMF638ServiceInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object, id) = path.into_inner();
    let mut tmf638 = tmf638.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf638.persist(persist.clone());
    match object.as_str() {
        "service" => {
            let product = tmf638.delete_service(id).await;
            render_delete_output(product)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
} 

pub fn config_tmf638(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf638 = TMF638ServiceInventoryManagement::new(None);
    cfg
        .service(tmf638_list_handler)
        .service(tmf638_get_handler)
        .service(tmf638_create_handler)
        .service(tmf638_patch_handler)
        .service(tmf638_delete_handler)
        .app_data(web::Data::new(Mutex::new(tmf638)));
}