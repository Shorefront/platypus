//! TMF633 Service Catalog Module

use std::sync::Mutex;

use actix_web::{get,patch, delete, post,web, HttpResponse, Responder};
use tmf633_service_catalog_management::TMF633ServiceCatalogManagement;
use tmflib::tmf633::service_candidate::ServiceCandidate;
use tmflib::tmf633::service_specification::ServiceSpecification;
use crate::common::persist::Persistence;
use crate::common::error::PlatypusError;
use crate::QueryOptions;

mod tmf633_service_catalog_management;

use crate::model::tmf::{
    render_list_output,
    // render_get_output,
    // render_post_output
    render_patch_output,
};

#[get("/tmf-api/serviceCatalogManagement/v4/{object}")]
pub async fn tmf633_list_handler(
        path : web::Path<String>,
        tmf633: web::Data<Mutex<TMF633ServiceCatalogManagement>>,
        persist: web::Data<Mutex<Persistence>>,
        query : web::Query<QueryOptions>,
    ) -> impl Responder {
    let object = path.into_inner();
    let query_opts = query.into_inner();
    let persist = persist.lock().unwrap();
    // Now have to pass persistence into tmf module here
    let mut tmf633 = tmf633.lock().unwrap();
    tmf633.persist(persist.clone());
    match object.as_str() {
        "serviceCandidate" => {
            let output = tmf633.get_candidates(query_opts).await;
            render_list_output(output)
        },
        // "serviceCatalog" => {
        //     let output = tmf633.get_catalogs(query_opts).await;
        //     render_list_output(output)
        // },
        // "serviceCategory" => {
        //     let output = tmf633.get_categories(query_opts).await;
        //     render_list_output(output)        
        // },
        "serviceSpecification" => {
            let output = tmf633.get_specifications(query_opts).await;
            render_list_output(output)        
        },
        _ => HttpResponse::BadRequest().json(PlatypusError::from("serviceCandidate object not implemented"))
    }
    
}

#[get("/tmf-api/serviceCatalogManagement/v4/{object}/{id}")]
pub async fn tmf633_get_handler() -> impl Responder {
    HttpResponse::BadRequest().json(PlatypusError::from("GET: Not implemented"))
}

#[patch("/tmf-api/serviceCatalogManagement/v4/{object}/{id}")]
pub async fn tmf633_patch_handler(
    path : web::Path<(String,String)>,
    tmf633: web::Data<Mutex<TMF633ServiceCatalogManagement>>,
    persist: web::Data<Mutex<Persistence>>,
    raw: web::Bytes,
) -> impl Responder {
    let (object,id) = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf633 = tmf633.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf633.persist(persist.clone());
    match object.as_str() {
        "serciceCandidate" => {
            let candidate : ServiceCandidate = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf633.update_candidate(id, candidate).await;
            render_patch_output(result)
        },
        "serviceSpecification" => {
            let specification : ServiceSpecification = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf633.update_specification(id, specification).await;
            render_patch_output(result)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("PATCH: Bad object: {object}"))
    } 
}

#[post("/tmf-api/serviceCatalogManagement/v4/{object}")]
pub async fn tmf633_post_handler() -> impl Responder {
    HttpResponse::BadRequest().json(PlatypusError::from("POST: Not implemented"))
}

#[delete("/tmf-api/serviceCatalogManagement/v4/{object}/{id}")]
pub async fn tmf633_delete_handler() -> impl Responder {
    HttpResponse::BadRequest().json(PlatypusError::from("DELETE: Not implemented"))
}

pub fn config_tmf633(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf633 = TMF633ServiceCatalogManagement::new();
    cfg
        .app_data(web::Data::new(Mutex::new(tmf633.clone())))
        .service(tmf633_list_handler)
        .service(tmf633_get_handler)
        .service(tmf633_patch_handler)
        .service(tmf633_post_handler)
        .service(tmf633_delete_handler);
}