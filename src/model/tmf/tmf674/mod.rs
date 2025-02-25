//! TMF674 Module

use std::sync::Mutex;
use tmf674_geographic_site::TMF674GeographicSiteManagement;
use tmflib::tmf674::geographic_site_v4::GeographicSite;
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

pub mod tmf674_geographic_site;

/// Get a list
#[get("/tmf-api/geographicSiteManagement/v4/{object}")]
pub async fn tmf674_list_handler(
    path : web::Path<String>,
    query : web::Query<QueryOptions>,
    tmf674: web::Data<Mutex<TMF674GeographicSiteManagement>>,
    persist: web::Data<Mutex<Persistence>>,  
) -> impl Responder {
    let object = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf674 = tmf674.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf674.persist(persist.clone());
    match object.as_str() {
        "geographicSite" => {
            let sites = tmf674.get_sites(query_opts).await;
            render_list_output(sites)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

#[get("/tmf-api/geographicSiteManagement/v4/{object}/{id}")]
pub async fn tmf674_get_handler(
    path : web::Path<(String,String)>,
    query : web::Query<QueryOptions>,
    tmf674: web::Data<Mutex<TMF674GeographicSiteManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object,id) = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf674 = tmf674.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf674.persist(persist.clone());
    match object.as_str() {
        "geographicSite" => {
            let customers = tmf674.get_site(id, query_opts).await;
            render_get_output(customers)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

/// Create an object
#[post("/tmf-api/geographicSiteManagement/v4/{object}")]
pub async fn tmf674_post_handler(
    path : web::Path<String>,
    raw: web::Bytes,
    tmf674: web::Data<Mutex<TMF674GeographicSiteManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let object = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf674 = tmf674.lock().unwrap();
    let persist = persist.lock().unwrap();
    // Set persistance into TMF object
    tmf674.persist(persist.clone());
    match object.as_str() {
        "geographicSite" => {
            let site : GeographicSite = serde_json::from_str(json.as_str()).expect("Could not parse TMF674 objet");
            let result = tmf674.add_site(site).await;
            render_post_output(result)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object: {object}"))
        }
    }
}

/// Update an object
#[patch("/tmf-api/geographicSiteManagement/v4/{object}/{id}")]
pub async fn tmf674_patch_handler(
    path : web::Path<(String,String)>,
    tmf674: web::Data<Mutex<TMF674GeographicSiteManagement>>,
    persist: web::Data<Mutex<Persistence>>,
    raw: web::Bytes,
) -> impl Responder {
    let (object,id) = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf674 = tmf674.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf674.persist(persist.clone());
    match object.as_str() {
        "site" => {
            let site : GeographicSite = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf674.update_site(id, site).await;
            render_patch_output(result)
        },
        _ => HttpResponse::BadRequest().json(PlatypusError::from("PATCH: Bad object: {object}"))
    } 
}


#[delete("/tmf-api/geographicSiteManagement/v4/{object}/{id}")]
pub async fn tmf674_delete_handler(
    path : web::Path<(String,String)>,
    tmf674: web::Data<Mutex<TMF674GeographicSiteManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object,id) = path.into_inner();
    let mut tmf674 = tmf674.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf674.persist(persist.clone());
    match object.as_str() {
        "geographicSite" => {
            let customers = tmf674.delete_site(id).await;
            render_delete_output(customers)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

pub fn config_tmf674(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf674 = TMF674GeographicSiteManagement::new(None);
    cfg
        .service(tmf674_list_handler)
        .service(tmf674_get_handler)
        .service(tmf674_post_handler)
        .service(tmf674_patch_handler)
        .service(tmf674_delete_handler)
        .app_data(web::Data::new(Mutex::new(tmf674.clone())));
}