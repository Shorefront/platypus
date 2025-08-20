//! TMF633 Service Catalog Module

use std::sync::Mutex;

use crate::common::error::PlatypusError;
use crate::common::persist::Persistence;
use crate::QueryOptions;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use tmf633_service_catalog_management::TMF633ServiceCatalogManagement;
use tmflib::tmf633::service_candidate::ServiceCandidate;
use tmflib::tmf633::service_catalog::ServiceCatalog;
use tmflib::tmf633::service_category::ServiceCategory;
use tmflib::tmf633::service_specification::ServiceSpecification;

mod tmf633_service_catalog_management;

use crate::model::tmf::{
    render_delete_output, render_get_output, render_list_output, render_patch_output,
    render_post_output,
};

/// Create an object
#[post("/tmf-api/serviceCatalogManagement/v4/{object}")]
pub async fn tmf633_post_handler(
    path: web::Path<String>,
    raw: web::Bytes,
    tmf633: web::Data<Mutex<TMF633ServiceCatalogManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let object = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf633 = tmf633.lock().unwrap();
    let persist = persist.lock().unwrap();
    // Set persistance into TMF object
    tmf633.persist(persist.clone());
    match object.as_str() {
        "serviceCandidate" => {
            let category: ServiceCandidate = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf633.create_candidate(category).await;
            render_post_output(result)
        }
        "serviceCatalog" => {
            let category: ServiceCatalog = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf633.create_catalog(category).await;
            render_post_output(result)
        }
        "serviceCategory" => {
            let category: ServiceCategory = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf633.create_category(category).await;
            render_post_output(result)
        }
        "serviceSpecification" => {
            let category: ServiceSpecification = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf633.create_specification(category).await;
            render_post_output(result)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from(
            "serviceCandidate object not implemented",
        )),
    }
}

#[get("/tmf-api/serviceCatalogManagement/v4/{object}")]
pub async fn tmf633_list_handler(
    path: web::Path<String>,
    tmf633: web::Data<Mutex<TMF633ServiceCatalogManagement>>,
    persist: web::Data<Mutex<Persistence>>,
    query: web::Query<QueryOptions>,
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
        }
        "serviceCatalog" => {
            let output = tmf633.get_catalogs(query_opts).await;
            render_list_output(output)
        }
        "serviceCategory" => {
            let output = tmf633.get_categories(query_opts).await;
            render_list_output(output)
        }
        "serviceSpecification" => {
            let output = tmf633.get_specifications(query_opts).await;
            render_list_output(output)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from(
            "serviceCandidate object not implemented",
        )),
    }
}

#[get("/tmf-api/serviceCatalogManagement/v4/{object}/{id}")]
pub async fn tmf633_get_handler(
    path: web::Path<(String, String)>,
    tmf633: web::Data<Mutex<TMF633ServiceCatalogManagement>>,
    persist: web::Data<Mutex<Persistence>>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    let (object, id) = path.into_inner();
    let query_opts = query.into_inner();
    let persist = persist.lock().unwrap();
    // Now have to pass persistence into tmf module here
    let mut tmf633 = tmf633.lock().unwrap();
    tmf633.persist(persist.clone());
    match object.as_str() {
        "serviceCandidate" => {
            let output = tmf633.get_candidate(id, query_opts).await;
            render_get_output(output)
        }
        "serviceCatalog" => {
            let output = tmf633.get_catalog(id, query_opts).await;
            render_get_output(output)
        }
        "serviceCategory" => {
            let output = tmf633.get_category(id, query_opts).await;
            render_get_output(output)
        }
        "serviceSpecification" => {
            let output = tmf633.get_specification(id, query_opts).await;
            render_get_output(output)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from(
            "serviceCandidate object not implemented",
        )),
    }
}

#[patch("/tmf-api/serviceCatalogManagement/v4/{object}/{id}")]
pub async fn tmf633_patch_handler(
    path: web::Path<(String, String)>,
    tmf633: web::Data<Mutex<TMF633ServiceCatalogManagement>>,
    persist: web::Data<Mutex<Persistence>>,
    raw: web::Bytes,
) -> impl Responder {
    let (object, id) = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf633 = tmf633.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf633.persist(persist.clone());
    match object.as_str() {
        "serciceCandidate" => {
            let candidate: ServiceCandidate = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf633.update_candidate(id, candidate).await;
            render_patch_output(result)
        }
        "serviceCatalog" => {
            let catalog: ServiceCatalog = serde_json::from_str(json.as_str()).unwrap();
            let output = tmf633.update_catalog(id, catalog).await;
            render_patch_output(output)
        }
        "serviceCategory" => {
            let category: ServiceCategory = serde_json::from_str(json.as_str()).unwrap();
            let output = tmf633.update_category(id, category).await;
            render_patch_output(output)
        }
        "serviceSpecification" => {
            let specification: ServiceSpecification = serde_json::from_str(json.as_str()).unwrap();
            let output = tmf633.update_specification(id, specification).await;
            render_patch_output(output)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("PATCH: Bad object: {object}")),
    }
}

#[delete("/tmf-api/serviceCatalogManagement/v4/{object}/{id}")]
pub async fn tmf633_delete_handler(
    path: web::Path<(String, String)>,
    tmf633: web::Data<Mutex<TMF633ServiceCatalogManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let (object, id) = path.into_inner();
    let mut tmf633 = tmf633.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf633.persist(persist.clone());
    match object.as_str() {
        "serviceCandidate" => {
            let output = tmf633.delete_candidate(id).await;
            render_delete_output(output)
        }
        "catalog" => {
            let output = tmf633.delete_catalog(id).await;
            render_delete_output(output)
        }
        "category" => {
            let output = tmf633.delete_category(id).await;
            render_delete_output(output)
        }
        "serviceSpecification" => {
            let output = tmf633.delete_specification(id).await;
            render_delete_output(output)
        }
        _ => HttpResponse::BadRequest().finish(),
    }
}

pub fn config_tmf633(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf633 = TMF633ServiceCatalogManagement::new();
    cfg.app_data(web::Data::new(Mutex::new(tmf633.clone())))
        .service(tmf633_list_handler)
        .service(tmf633_get_handler)
        .service(tmf633_patch_handler)
        .service(tmf633_post_handler)
        .service(tmf633_delete_handler);
}
