//! TMC632 Module
//! 

use std::sync::Mutex;
use tmf632_party_management::TMF632PartyManagement;
use actix_web::{get,post,web, HttpResponse, Responder};

// TMFLIB
#[cfg(feature = "tmf632_v4")]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(feature = "tmf632_v5")]
use tmflib::tmf632::individual_v5::Individual;
#[cfg(feature = "tmf632_v4")]
use tmflib::tmf632::organization_v4::Organization;
#[cfg(feature = "tmf632_v5")]
use tmflib::tmf632::organization_v5::Organization;
use tmflib::HasId;

use crate::common::error::PlatypusError;
use crate::common::persist::Persistence;
use crate::QueryOptions;
use crate::model::tmf::{
    render_list_output,
    render_get_output,
    render_post_output
};

#[cfg(feature = "tmf632_v4")]
pub mod tmf632_party_management;

#[get("/tmf-api/partyManagement/v4/{object}")]
pub async fn tmf632_list_handler(
    path : web::Path<String>,
    tmf632: web::Data<Mutex<TMF632PartyManagement>>,
    query : web::Query<QueryOptions>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let query_opts = query.into_inner();
    let mut tmf632 = tmf632.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf632.persist(persist.clone());
    match path.as_str() {
        "individual" => {
            let result = tmf632.get_individuals(query_opts).await;
            render_list_output(result) 
        },
        "organization" => todo!(),
        _ => HttpResponse::BadRequest().json(PlatypusError::from("TMF632: Invalid Object"))
    }  
}

/// Get a specific object
#[get("/tmf-api/partyManagement/v4/{object}/{id}")]
pub async fn tmf632_get_handler(
    path : web::Path<(String,String)>,
    tmf632: web::Data<Mutex<TMF632PartyManagement>>,
    query : web::Query<QueryOptions>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let (object,id) = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf632 = tmf632.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf632.persist(persist.clone());
    match object.as_str() {
        "individual" => {
            let result = tmf632.get_individual(id,query_opts).await;
            render_get_output(result)      
        },
        _ => HttpResponse::BadRequest().json(PlatypusError::from("TMF632: Invalid Object"))    
    }
}

#[post("/tmf-api/partyManagement/v4/{object}")]
pub async fn tmf632_post_handler(
    path : web::Path<String>,
    raw: web::Bytes,
    tmf632: web::Data<Mutex<TMF632PartyManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let object = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf632 = tmf632.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf632.persist(persist.clone());
    match object.as_str() {
        "individual" => {
            // Create individual object
            let mut individual : Individual = serde_json::from_str(json.as_str()).unwrap();
            individual.generate_id();
            let records = tmf632.add_individual(individual.clone()).await;
            render_post_output(records)
        },
        "organization" => {
            let mut organization : Organization = serde_json::from_str(json.as_str()).unwrap();
            organization.generate_id();
            let records = tmf632.add_organization(organization.clone()).await;
            render_post_output(records)
        }
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("TMF632: Invalid Object"))
        }
    } 
}

pub fn config_tmf632(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf632 = TMF632PartyManagement::new(None);
    cfg
        .service(tmf632_list_handler)
        .service(tmf632_get_handler)
        .service(tmf632_post_handler)
        .app_data(web::Data::new(Mutex::new(tmf632.clone())));
}