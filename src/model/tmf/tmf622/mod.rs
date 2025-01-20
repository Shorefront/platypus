//! TMF622 Module
//! 

pub mod tmf622_product_order_management;

use std::sync::Mutex;
use tmf622_product_order_management::TMF622ProductOrderManagement;
use tmflib::tmf622::product_order_v4::ProductOrder;
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
    render_list_output,
    render_post_output,
    render_patch_output,
    render_delete_output,
};

/// Get a list
#[get("tmf-api/productOrderingManagement/v4/{object}")]
pub async fn tmf622_list_handler(
    path : web::Path<String>,
    query : web::Query<QueryOptions>,
    tmf622: web::Data<Mutex<TMF622ProductOrderManagement>>,
    persist: web::Data<Mutex<Persistence>>,  
) -> impl Responder {
    let object = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf622 = tmf622.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf622.persist(persist.clone());
    match object.as_str() {
        "productOrder" => {
            let sites = tmf622.get_orders(query_opts).await;
            render_list_output(sites)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

#[get("/tmf-api/productOrderingManagement/v4/{object}/{id}")]
pub async fn tmf622_get_handler(
    path : web::Path<(String,String)>,
    query : web::Query<QueryOptions>,
    tmf674: web::Data<Mutex<TMF622ProductOrderManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object,id) = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf674 = tmf674.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf674.persist(persist.clone());
    match object.as_str() {
        "productOrder" => {
            let customers = tmf674.get_order(id, query_opts).await;
            render_list_output(customers)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

/// Create an object
#[post("/tmf-api/productOrderingManagement/v4/{object}")]
pub async fn tmf622_post_handler(
    path : web::Path<String>,
    raw: web::Bytes,
    tmf622: web::Data<Mutex<TMF622ProductOrderManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let object = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf622 = tmf622.lock().unwrap();
    let persist = persist.lock().unwrap();
    // Set persistance into TMF object
    tmf622.persist(persist.clone());
    match object.as_str() {
        "productOrder" => {
            let order : ProductOrder = serde_json::from_str(json.as_str()).expect("Could not parse TMF622 objet");
            let result = tmf622.add_order(order).await;
            render_post_output(result)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object: {object}"))
        }
    }
}

/// Update an object
#[patch("/tmf-api/productOrderingManagement/v4/{object}/{id}")]
pub async fn tmf622_patch_handler(
    path : web::Path<(String,String)>,
    tmf622: web::Data<Mutex<TMF622ProductOrderManagement>>,
    persist: web::Data<Mutex<Persistence>>,
    raw: web::Bytes,
) -> impl Responder {
    let (object,id) = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf622 = tmf622.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf622.persist(persist.clone());
    match object.as_str() {
        "productOrder" => {
            let order : ProductOrder = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf622.update_order(id, order).await;
            render_patch_output(result)
        },
        _ => HttpResponse::BadRequest().json(PlatypusError::from("PATCH: Bad object: {object}"))
    } 
}


#[delete("/tmf-api/productOrderingManagement/v4/{object}/{id}")]
pub async fn tmf622_delete_handler(
    path : web::Path<(String,String)>,
    tmf622: web::Data<Mutex<TMF622ProductOrderManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object,id) = path.into_inner();
    let mut tmf622 = tmf622.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf622.persist(persist.clone());
    match object.as_str() {
        "productOrder" => {
            let customers = tmf622.delete_order(id).await;
            render_delete_output(customers)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

pub fn config_tmf622(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf622 = TMF622ProductOrderManagement::new(None);
    cfg
        .service(tmf622_list_handler)
        .service(tmf622_get_handler)
        .service(tmf622_post_handler)
        .service(tmf622_patch_handler)
        .service(tmf622_delete_handler)
        .app_data(web::Data::new(Mutex::new(tmf622.clone())));
}