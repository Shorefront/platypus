//! Platypus Priary Module

#![warn(missing_docs)]

use log::info;

mod model;
mod template;
mod common;

use actix_web::middleware::Logger;
use actix_web::{get,post,patch,delete,web,App, HttpResponse,HttpServer, Responder};

use log::error;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::product_offering_price::ProductOfferingPrice;

use std::sync::Mutex;

// SurrealDB
use serde::Deserialize;
//use surrealdb::engine::local::Mem;
use surrealdb::engine::local::Db;
use surrealdb::engine::local::SpeeDb;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

// TMFLIB
use common::config::Config;
use common::error::PlatypusError;
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::category::Category;
use tmflib::tmf620::product_specification::ProductSpecification;
use tmflib::tmf632::individual::Individual;
use tmflib::tmf632::organization::Organization;
use tmflib::tmf629::customer::Customer;
use tmflib::tmf629::customer::CUST_STATUS;
use tmflib::tmf648::quote::Quote;
use tmflib::{HasId, HasLastUpdate};

//use crate::template::product::ProductTemplate;
//use crate::model::component::product::ProductComponent;
use crate::model::tmf::tmf620_catalog_management::TMF620CatalogManagement;
use crate::model::tmf::tmf632_party_management::TMF632PartyManagement;

#[derive(Debug,Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Option<Thing>,
}

/// Get a list
#[get("/tmf-api/productCatalogManagement/v4/{object}")]
pub async fn tmf620_list_handler(
    path : web::Path<String>,
    tmf620: web::Data<Mutex<TMF620CatalogManagement>>
) -> impl Responder {
    let object = path.into_inner();
    match object.as_str() {
        "productSpecification" => {
            let output = tmf620.lock().unwrap().get_specifications().await;
            match output {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),
            }
        },
        "productOffering" => {
            let output = tmf620.lock().unwrap().get_offers().await;
            match output {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),
            }
        }
        "productOfferingPrice" => {
            let output = tmf620.lock().unwrap().get_prices().await;
            match output {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),
            }    
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Bad Object: {object")),
    }
}

/// Get a specific object
#[get("/tmf-api/productCatalogManagement/v4/{object}/{id}")]
pub async fn tmf620_get_handler(
    path : web::Path<(String,String)>,
    tmf620: web::Data<Mutex<TMF620CatalogManagement>>
) -> impl Responder {
    let (object,id) = path.into_inner();
    match object.as_str() {
        "productSpecification" => {
            let data = tmf620.lock().unwrap().get_specification(id).await;
            match data {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),    
            }
        },
        "productOffering" => {
            let data = tmf620.lock().unwrap().get_offer(id).await;
            match data {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),    
            }
        },
        "productOfferingPrice" => {
            let data = tmf620.lock().unwrap().get_price(id).await;
            match data {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),    
            }
        },
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object"))
    }
}

/// Create an object
#[post("/tmf-api/productCatalogManagement/v4/{object}")]
pub async fn tmf620_post_handler(
    path : web::Path<String>,
    raw: web::Bytes,
    tmf620: web::Data<Mutex<TMF620CatalogManagement>>
) -> impl Responder {
    let object = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    match object.as_str() {
        // Create specification 
        "productSpecification" => {
            let mut specification : ProductSpecification = serde_json::from_str(json.as_str()).unwrap();
            if specification.id.is_none() {
                specification.generate_id();
            }
            // Set last update for new records
            specification.set_last_update(ProductSpecification::get_timestamp());
            let result = tmf620.lock().unwrap().add_specification(specification).await;
            match result {
                Ok(r) => {
                    //let json = serde_json::to_string(
                    let item = r.first().unwrap().clone();
                    HttpResponse::Created().json(item)
                },
                Err(e) => HttpResponse::BadRequest().json(e),
            }
        },
        "productOffering" => {
            let mut offering : ProductOffering = serde_json::from_str(json.as_str())
                .expect("Could not parse ProductOffering");
            if offering.id.is_none() {
                offering.generate_id();
            }
            // Set last update for new records
            offering.set_last_update(ProductOffering::get_timestamp());
            let result = tmf620.lock().unwrap().add_offering(offering).await;
            match result {
                Ok(r) => {
                    let item = r.first().unwrap().clone();
                    HttpResponse::Created().json(item)
                },
                Err(e) => HttpResponse::BadGateway().json(e),
            }
        },
        "productOfferingPrice" => {
            let mut price : ProductOfferingPrice = serde_json::from_str(json.as_str())
                .expect("Could not parse productOfferingPrice");
            if price.id.is_none() {
                price.generate_id();
            }
            // Set last update for new records
            price.set_last_update(ProductOfferingPrice::get_timestamp());
            let result = tmf620.lock().unwrap().add_price(price).await;
            match result {
                Ok(r) => {
                    let item = r.first().unwrap().clone();
                    HttpResponse::Created().json(item)
                },
                Err(e) => HttpResponse::BadGateway().json(e),
            }
        }
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object: {object}"))
        }
    }
}

/// Update an object
#[patch("/tmf-api/productCatalogManagement/v4/{object}/{id}")]
pub async fn tmf620_patch_handler(
    path : web::Path<(String,String)>,
    raw: web::Bytes,
    tmf620: web::Data<Mutex<TMF620CatalogManagement>>
) -> impl Responder {
    let (object,id) = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    match object.as_str() {
        "productSpecification" => {
            match tmf620.lock().unwrap().patch_specification(id,json).await {
                Ok(r) => HttpResponse::Ok().json(r),
                Err(e) => {
                    error!("Could not delete: {e}");
                    HttpResponse::BadRequest().json(e)
                },
            }
        },
        "productOffering" => {
            match tmf620.lock().unwrap().patch_offering(id,json).await {
                Ok(r) => HttpResponse::Ok().json(r),
                Err(e) => {
                    error!("Could not delete: {e}");
                    HttpResponse::BadRequest().json(PlatypusError::from("PATCH: Bad object"))
                },
            }
        },
        "productOfferingPrice"  => {
            match tmf620.lock().unwrap().patch_price(id,json).await {
                Ok(r) => HttpResponse::Ok().json(r),
                Err(e) => {
                    error!("Could not delete: {e}");
                    HttpResponse::BadRequest().json(PlatypusError::from("PATCH: Bad object"))
                },
            }
        },
        _ => HttpResponse::BadRequest().json(PlatypusError::from("PATCH: Bad object"))
    } 
}

/// Detele an object
#[delete("/tmf-api/productCatalogManagement/v4/{object}/{id}")]
pub async fn tmf620_delete_handler(
    path : web::Path<(String,String)>,
    tmf620: web::Data<Mutex<TMF620CatalogManagement>>
) -> impl Responder {
    let (object,id) = path.into_inner();
    match object.as_str() {
        "productSpecification" => {
            match tmf620.lock().unwrap().delete_specification(id).await {
                Ok(_b) => HttpResponse::NoContent(),
                Err(e) => {
                    error!("Could not delete: {e}");
                    HttpResponse::BadRequest()
                },
            }
        },
        "productOffering" => {
            match tmf620.lock().unwrap().delete_offering(id).await {
                Ok(_b) => HttpResponse::NoContent(),
                Err(e) => {
                    error!("Could not delete: {e}");
                    HttpResponse::BadRequest()
                },
            }
        },
        "productOfferingPrice"  => {
            match tmf620.lock().unwrap().delete_price(id).await {
                Ok(_b) => HttpResponse::NoContent(),
                Err(e) => {
                    error!("Could not delete: {e}");
                    HttpResponse::BadRequest()
                },
            }
        },
        _ => HttpResponse::BadRequest(),
    }  
}

#[post("/tmflib/tmf629/customer")]
pub async fn tmf629_create_handler(
    body : web::Json<Customer>,
    _db   : web::Data<Surreal<Db>>
) -> impl Responder {
    let mut data = body.into_inner();
    data.generate_code();
    // Since this a new customer we have to regenerate the id / href
    data.generate_id();
    data.status = Some(CUST_STATUS.to_string());
    HttpResponse::Ok().json(data)
}

#[get("/tmflib/tmf629/customer/{id}")]
pub async fn tmf629_get_handler(

) -> impl Responder {
    HttpResponse::Ok()
}

#[get("/tmflib/tmf632/{object}")]
pub async fn tmf632_get_handler(
    path : web::Path<String>,
    tmf632: web::Data<Mutex<TMF632PartyManagement>>,
) -> impl Responder {
    match path.as_str() {
        "individual" => {
            let result = tmf632.lock().unwrap().get_individuals().await;
            match result {
                Ok(v) => HttpResponse::Ok().json(v),
                Err(e) => HttpResponse::BadRequest().json(e),
            }   
        },
        "organization" => todo!(),
        _ => HttpResponse::BadRequest().json(PlatypusError::from("TMF632: Invalid Object"))
    }  
}

#[post("/tmflib/tmf632/{object}")]
pub async fn tmf632_create_handler(
    path : web::Path<String>,
    raw: web::Bytes,
    tmf632: web::Data<Mutex<TMF632PartyManagement>>,
) -> impl Responder {
    let object = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    match object.as_str() {
        "individual" => {
            // Create individual object
            let mut individual : Individual = serde_json::from_str(json.as_str()).unwrap();
            individual.generate_id();
            let records = tmf632.lock().unwrap().add_individual(individual.clone()).await;
            match records {
                Ok(r) => HttpResponse::Ok().json(r),
                Err(e) => HttpResponse::BadRequest().json(e),
            } 
        },
        "organization" => {
            let mut organization : Organization = serde_json::from_str(json.as_str()).unwrap();
            organization.generate_id();
            HttpResponse::Ok().json(organization)
        }
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("TMF632: Invalid Object"))
        }
    } 
}

#[post("/tmflib/tmf648/quote")]
pub async fn tmf648_create_handler(
    body : web::Json<Quote>
) -> impl Responder {
    let data = body.into_inner();
    HttpResponse::Ok().json(data)
}

#[warn(missing_docs)]

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    env_logger::init();

    info!("Starting {pkg} v{ver}");

    let db = Surreal::new::<SpeeDb>("/home/rruckley/build/platypus/tmf.db").await.expect("Could not create DB");

    db.use_ns("tmflib").use_db("composable").await.expect("Could not set DB NS");

    let tmf620 = TMF620CatalogManagement::new(db.clone());
    let tmf632 = TMF632PartyManagement::new(db.clone());

    let config = Config::new();

    // Extract port crom config, default if not found
    let port = config.get("PLATYPUS_PORT").unwrap_or("8000".to_string());
    let port = port.parse::<u16>().unwrap();
   
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(tmf620.clone())))
            .app_data(web::Data::new(Mutex::new(tmf632.clone())))
            .app_data(web::Data::new(Mutex::new(config.clone())))
            .service(tmf620_post_handler)
            .service(tmf620_list_handler)
            .service(tmf620_get_handler)
            .service(tmf620_patch_handler)
            .service(tmf620_delete_handler)
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0",port))?
        .run()
        .await
}
