//! TMF629 Module

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
    // render_get_output,
    render_list_output,
    render_post_output,
    render_patch_output,
    render_delete_output,
};

// TMFLIB
use tmflib::tmf629::customer::Customer;


pub mod tmf629_customer_management;
use tmf629_customer_management::TMF629CustomerManagement;

#[get("/tmf-api/customerManagement/v4/{object}")]
pub async fn tmf629_list_handler(
    path : web::Path<String>,
    query : web::Query<QueryOptions>,
    tmf629: web::Data<Mutex<TMF629CustomerManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let object = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf629 = tmf629.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf629.persist(persist.clone());
    match object.as_str() {
        "customer" => {
            let customers = tmf629.get_customers(query_opts).await;
            render_list_output(customers)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

#[get("/tmf-api/customerManagement/v4/{object}/{id}")]
pub async fn tmf629_get_handler(
    path : web::Path<(String,String)>,
    query : web::Query<QueryOptions>,
    tmf629: web::Data<Mutex<TMF629CustomerManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object,id) = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf629 = tmf629.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf629.persist(persist.clone());
    match object.as_str() {
        "customer" => {
            let customers = tmf629.get_customer(id, query_opts).await;
            render_list_output(customers)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

/// Create an object
#[post("/tmf-api/customerManagement/v4/{object}")]
pub async fn tmf629_create_handler(
    path : web::Path<String>,
    raw: web::Bytes,
    tmf629: web::Data<Mutex<TMF629CustomerManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let object = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf629 = tmf629.lock().unwrap();
    let persist = persist.lock().unwrap();
    // Set persistance into TMF object
    tmf629.persist(persist.clone());
    match object.as_str() {
        "customer" => {
            let customer : Customer = serde_json::from_str(json.as_str()).expect("Could not parse TMF674 objet");
            let result = tmf629.add_customer(customer).await;
            render_post_output(result)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object: {object}"))
        }
    }
}

/// Update an object
#[patch("/tmf-api/customerManagement/v4/{object}/{id}")]
pub async fn tmf629_patch_handler(
    path : web::Path<(String,String)>,
    tmf629: web::Data<Mutex<TMF629CustomerManagement>>,
    persist: web::Data<Mutex<Persistence>>,
    raw: web::Bytes,
) -> impl Responder {
    let (object,id) = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf629 = tmf629.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf629.persist(persist.clone());
    match object.as_str() {
        "customer" => {
            let customer : Customer = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf629.update_customer(id, customer).await;
            render_patch_output(result)
        },
        _ => HttpResponse::BadRequest().json(PlatypusError::from("PATCH: Bad object: {object}"))
    } 
}


#[delete("/tmf-api/customerManagement/v4/{object}/{id}")]
pub async fn tmf629_delete_handler(
    path : web::Path<(String,String)>,
    tmf629: web::Data<Mutex<TMF629CustomerManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object,id) = path.into_inner();
    let mut tmf629 = tmf629.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf629.persist(persist.clone());
    match object.as_str() {
        "customer" => {
            let customers = tmf629.delete_customer(id).await;
            render_delete_output(customers)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

pub fn config_tmf629(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf629 = TMF629CustomerManagement::new(None);
    cfg
        .service(tmf629_list_handler)
        .service(tmf629_get_handler)
        .service(tmf629_create_handler)
        .service(tmf629_delete_handler)
        .app_data(web::Data::new(Mutex::new(tmf629)));
}