//! TMF645 Service Qualification Module
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

// TMFLib
use tmflib::tmf645::check_service_qualification::CheckServiceQualification;

pub mod tmf645_service_qualification_management;
use tmf645_service_qualification_management::TMF645ServiceQualificationManagement;

#[post("/tmf-api/serviceQualificationManagement/v4/{object}")]
pub async fn tmf645_create_handler(
    path : web::Path<String>,
    raw: web::Bytes,
    tmf645: web::Data<Mutex<TMF645ServiceQualificationManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let object = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf645 = tmf645.lock().unwrap();
    let persist = persist.lock().unwrap();
    // Set persistance into TMF object
    tmf645.persist(persist.clone());
    match object.as_str() {
        "checkServiceQualification" => {
            let qualification : CheckServiceQualification = serde_json::from_str(json.as_str()).expect("Could not parse TMF648 objet");
            let result = tmf645.add_check_qualification(qualification).await;
            render_post_output(result)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object: {object}"))
        }
    }
}

#[patch("/tmf-api/serviceQualificationManagement/v4/{object}/{id}")]
pub async fn tmf645_patch_handler(
    path : web::Path<(String,String)>,
    tmf645: web::Data<Mutex<TMF645ServiceQualificationManagement>>,
    persist: web::Data<Mutex<Persistence>>,
    raw: web::Bytes,
) -> impl Responder {
    let (object,id) = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf645 = tmf645.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf645.persist(persist.clone());
    match object.as_str() {
        "checkServiceQualification" => {
            let qualification : CheckServiceQualification = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf645.update_check_qualification(id, qualification).await;
            render_patch_output(result)
        },
        _ => HttpResponse::BadRequest().json(PlatypusError::from("PATCH: Bad object: {object}"))
    } 
}

#[get("/tmf-api/serviceQualificationManagement/v4/{object}")]
pub async fn tmf645_list_handler(
    path : web::Path<String>,
    query : web::Query<QueryOptions>,
    tmf645: web::Data<Mutex<TMF645ServiceQualificationManagement>>,
    persist: web::Data<Mutex<Persistence>>,    
) -> impl Responder {
    let object = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf645 = tmf645.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf645.persist(persist.clone());
    match object.as_str() {
        "checkServiceQualification" => {
            let qualifications = tmf645.get_check_qualifications(query_opts).await;
            render_list_output(qualifications)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

#[get("/tmf-api/serviceQualificationManagement/v4/{object}/{id}")]
pub async fn tmf645_get_handler(
    path : web::Path<(String,String)>,
    query : web::Query<QueryOptions>,
    tmf645: web::Data<Mutex<TMF645ServiceQualificationManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object,id) = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf645 = tmf645.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf645.persist(persist.clone());
    match object.as_str() {
        "checkServiceQualification" => {
            let qualifications = tmf645.get_check_qualification(id, query_opts).await;
            render_get_output(qualifications)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

#[delete("/tmf-api/serviceQualificationManagement/v4/{object}/{id}")]
pub async fn tmf645_delete_handler(
    path : web::Path<(String,String)>,
    tmf645: web::Data<Mutex<TMF645ServiceQualificationManagement>>,
    persist: web::Data<Mutex<Persistence>>, 
) -> impl Responder {
    let (object,id) = path.into_inner();
    let mut tmf645 = tmf645.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf645.persist(persist.clone());
    match object.as_str() {
        "checkServiceQualification" => {
            let qualification = tmf645.delete_check_qualification(id).await;
            render_delete_output(qualification)
        },
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))   
        }
    } 
}

pub fn config_tmf645(cfg: &mut web::ServiceConfig) {
    let tmf645 = TMF645ServiceQualificationManagement::new(None);
    cfg
    .service(tmf645_list_handler)
    .service(tmf645_get_handler)
    .service(tmf645_create_handler)
    .service(tmf645_patch_handler)
    .service(tmf645_delete_handler)
    .app_data(web::Data::new(Mutex::new(tmf645)));
}