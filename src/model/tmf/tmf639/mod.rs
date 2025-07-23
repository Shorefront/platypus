//! TMF639 Resource Inventory
//! 

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

use tmflib::tmf639::resource::Resource;

pub mod tmf639_resource_inventory_management;
use tmf639_resource_inventory_management::TMF639ResourceInventoryManagement;


#[get("/tmf-api/resourceInventoryManagement/v4/{object}")]
pub async fn tmf639_list_handler(
    path : web::Path<String>,
    query : web::Query<QueryOptions>,
    tmf639: web::Data<Mutex<TMF639ResourceInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let object = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf639 = tmf639.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf639.persist(persist.clone());
    match object.as_str() {
        "resource" => {
            let products = tmf639.get_resources(query_opts).await;
            render_list_output(products)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

#[get("/tmf-api/resourceInventoryManagement/v4/{object}/{id}")]
pub async fn tmf639_get_handler(
    path : web::Path<(String,String)>,
    query : web::Query<QueryOptions>,
    tmf638: web::Data<Mutex<TMF639ResourceInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object, id) = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf638 = tmf638.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf638.persist(persist.clone());
    match object.as_str() {
        "resource" => {
            let product = tmf638.get_resource(id, query_opts).await;
            render_get_output(product)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}   

#[post("/tmf-api/resourceInventoryManagement/v4/{object}")]
pub async fn tmf639_create_handler(
    path : web::Path<String>,
    item: web::Json<Resource>,
    tmf638: web::Data<Mutex<TMF639ResourceInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let object = path.into_inner();
    let mut tmf638 = tmf638.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf638.persist(persist.clone());
    match object.as_str() {
        "resouce" => {
            let product = tmf638.add_resource(item.into_inner()).await;
            render_post_output(product)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}   

#[patch("/tmf-api/resourceInventoryManagement/v4/{object}/{id}")]
pub async fn tmf639_patch_handler(
    path : web::Path<(String,String)>,
    item: web::Json<Resource>,
    tmf637: web::Data<Mutex<TMF639ResourceInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object, id) = path.into_inner();
    let mut tmf637 = tmf637.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf637.persist(persist.clone());
    match object.as_str() {
        "resource" => {
            let product = tmf637.update_resource(id, item.into_inner()).await;
            render_patch_output(product)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

#[delete("/tmf-api/resourceInventoryManagement/v4/{object}/{id}")]
pub async fn tmf639_delete_handler(
    path : web::Path<(String,String)>,
    tmf638: web::Data<Mutex<TMF639ResourceInventoryManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object, id) = path.into_inner();
    let mut tmf638 = tmf638.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf638.persist(persist.clone());
    match object.as_str() {
        "resource" => {
            let product = tmf638.delete_resource(id).await;
            render_delete_output(product)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
} 

pub fn config_tmf639(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf639 = TMF639ResourceInventoryManagement::new(None);
    cfg
        .service(tmf639_list_handler)
        .service(tmf639_get_handler)
        .service(tmf639_create_handler)
        .service(tmf639_patch_handler)
        .service(tmf639_delete_handler)
        .app_data(web::Data::new(Mutex::new(tmf639)));
}