//! Platypus Primary Module

#![warn(missing_docs)]

use log::info;

mod model;
#[cfg(feature = "composable")]
mod template;
mod common;

use actix_web::middleware::Logger;
use actix_web::{get,post,patch,delete,web,App, HttpResponse,HttpServer, Responder};

use log::error;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::product_offering_price::ProductOfferingPrice;
#[cfg(feature = "tmf648_v4")]
use tmflib::tmf648::quote::Quote;
#[cfg(feature = "tmf674_v4")]
use tmflib::tmf674::geographic_site_v4::GeographicSite;
#[cfg(feature = "tmf674_v5")]
use tmflib::tmf674::geographic_site_v5::GeographicSite;

use std::sync::Mutex;

// SurrealDB
use serde::Deserialize;

// New Persistence struct
use common::persist::Persistence;

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
use tmflib::{HasId, HasLastUpdate};

#[cfg(feature = "composable")]
use crate::model::component::*;
#[cfg(feature = "composable")]
use crate::template::*;

//use crate::template::product::ProductTemplate;
//use crate::model::component::product::ProductComponent;
use crate::model::tmf::tmf620_catalog_management::TMF620CatalogManagement;
use crate::model::tmf::tmf632_party_management::TMF632PartyManagement;
use crate::model::tmf::tmf674_geographic_site::TMF674GeographicSiteManagement;

/// Fields for filtering output
#[derive(Clone, Debug, Deserialize)]
pub struct QueryOptions {
    /// Specific set of fields delimited by comma
    fields : Option<String>,
    limit : Option<u16>,
    offset : Option<u16>,
    /// Filter on name
    name : Option<String>,
}

/// Get a list
#[get("/tmf-api/productCatalogManagement/v4/{object}")]
pub async fn tmf620_list_handler(
    path : web::Path<String>,
    tmf620: web::Data<Mutex<TMF620CatalogManagement>>,
    query : web::Query<QueryOptions>,
) -> impl Responder {
    let object = path.into_inner();
    let query_opts = query.into_inner();

    match object.as_str() {
        "catalog" => {
            let output = tmf620.lock().unwrap().get_catalogs(query_opts).await;
            match output {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),
            }
        },
        "category" => {
            let output = tmf620.lock().unwrap().get_categories(query_opts).await;
            match output {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),
            }
        },
        "productSpecification" => {
            let output = tmf620.lock().unwrap().get_specifications(query_opts).await;
            match output {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),
            }
        },
        "productOffering" => {
            let output = tmf620.lock().unwrap().get_offers(query_opts).await;
            match output {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),
            }
        }
        "productOfferingPrice" => {
            let output = tmf620.lock().unwrap().get_prices(query_opts).await;
            match output {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),
            }    
        },
        "importJob" => {
            HttpResponse::BadRequest().json(PlatypusError::from("importJob: Not implemented"))
        },
        "exportJob" => {
            HttpResponse::BadRequest().json(PlatypusError::from("exportJob: Not implemented"))
        },
        "hub" => {
            HttpResponse::BadRequest().json(PlatypusError::from("Hub: Not implemented"))
        },
        "listener" => {
            HttpResponse::BadRequest().json(PlatypusError::from("listener: Not implemented"))
        },
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Bad Object: {object}")),
    }
}

/// Get a specific object
#[get("/tmf-api/productCatalogManagement/v4/{object}/{id}")]
pub async fn tmf620_get_handler(
    path : web::Path<(String,String)>,
    tmf620: web::Data<Mutex<TMF620CatalogManagement>>,
    query : web::Query<QueryOptions>,
) -> impl Responder {
    let (object,id) = path.into_inner();
    let query_opts = query.into_inner();
    
    match object.as_str() {
        "catalog" => {
            let output = tmf620.lock().unwrap().get_catalog(id,query_opts).await;
            match output {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),
            }
        },
        "category" => {
            let output = tmf620.lock().unwrap().get_category(id,query_opts).await;
            match output {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),
            }
        },
        "productSpecification" => {
            let data = tmf620.lock().unwrap().get_specification(id,query_opts).await;
            match data {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),    
            }
        },
        "productOffering" => {
            let data = tmf620.lock().unwrap().get_offer(id,query_opts).await;
            match data {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),    
            }
        },
        "productOfferingPrice" => {
            let data = tmf620.lock().unwrap().get_price(id,query_opts).await;
            match data {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),    
            }
        },
        "importJob" => {
            HttpResponse::BadRequest().json(PlatypusError::from("importJob: Not implemented"))
        },
        "exportJob" => {
            HttpResponse::BadRequest().json(PlatypusError::from("exportJob: Not implemented"))
        },
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object: {object}"))
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
        "category" => {
            let category : Category = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf620.lock().unwrap().add_category(category).await;
            match result {
                Ok(r) => {
                    //let json = serde_json::to_string(
                    let item = r.first().unwrap().clone();
                    HttpResponse::Created().json(item)
                },
                Err(e) => HttpResponse::BadRequest().json(e),
            }
        },
        "catalog" => {
            let catalog : Catalog = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf620.lock().unwrap().add_catalog(catalog).await;
            match result {
                Ok(r) => {
                    //let json = serde_json::to_string(
                    let item = r.first().unwrap().clone();
                    HttpResponse::Created().json(item)
                },
                Err(e) => HttpResponse::BadRequest().json(e),
            }
        }
        "productSpecification" => {
            let mut specification : ProductSpecification = serde_json::from_str(json.as_str()).unwrap();
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
        },
        "importJob" => {
            HttpResponse::BadRequest().json(PlatypusError::from("importJob: Not implemented"))
        },
        "exportJob" => {
            HttpResponse::BadRequest().json(PlatypusError::from("exportJob: Not implemented"))
        },
        "hub" => {
            HttpResponse::BadRequest().json(PlatypusError::from("Hub: Not implemented"))
        },
        "listener" => {
            HttpResponse::BadRequest().json(PlatypusError::from("listener: Not implemented"))
        },
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
        _ => HttpResponse::BadRequest().json(PlatypusError::from("PATCH: Bad object: {object}"))
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
        "catalog" => {
            match tmf620.lock().unwrap().delete_catalog(id).await {
                Ok(_b) => HttpResponse::NoContent(),
                Err(e) => {
                    error!("Could not delete: {e}");
                    HttpResponse::BadRequest()
                },     
            }    
        },
        "category" => {
            match tmf620.lock().unwrap().delete_category(id).await {
                Ok(_b) => HttpResponse::NoContent(),
                Err(e) => {
                    error!("Could not delete: {e}");
                    HttpResponse::BadRequest()
                },     
            }    
        },
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
) -> impl Responder {
    let mut data = body.into_inner();
    // Since this a new customer we have to regenerate the id / href
    data.generate_id();
    // Now that we have an id, we can generate a new code.
    data.generate_code(None);
    data.status = Some(CUST_STATUS.to_string());
    HttpResponse::Ok().json(data)
}

#[get("/tmflib/tmf629/customer/{id}")]
pub async fn tmf629_get_handler(

) -> impl Responder {
    HttpResponse::Ok()
}

#[get("/tmflib/tmf632/{object}")]
pub async fn tmf632_list_handler(
    path : web::Path<String>,
    tmf632: web::Data<Mutex<TMF632PartyManagement>>,
    query : web::Query<QueryOptions>,
) -> impl Responder {
    let query_opts = query.into_inner();
    match path.as_str() {
        "individual" => {
            let result = tmf632.lock().unwrap().get_individuals(query_opts).await;
            match result {
                Ok(v) => HttpResponse::Ok().json(v),
                Err(e) => HttpResponse::BadRequest().json(e),
            }   
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
) -> impl Responder {
    let (object,id) = path.into_inner();
    let query_opts = query.into_inner();
    match object.as_str() {
        "individual" => {
            let result = tmf632.lock().unwrap().get_individual(id,query_opts).await;
            match result {
                Ok(v) => HttpResponse::Ok().json(v),
                Err(e) => HttpResponse::BadRequest().json(e),
            }       
        },
        _ => HttpResponse::BadRequest().json(PlatypusError::from("TMF632: Invalid Object"))    
    }
}

#[post("/tmflib/tmf632/{object}")]
pub async fn tmf632_post_handler(
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

/// Create an Geographic Site object
#[post("/tmf-api/geographicSiteManagement/v4/{object}")]
pub async fn tmf674_post_handler(
    path : web::Path<String>,
    raw: web::Bytes,
    tmf674: web::Data<Mutex<TMF674GeographicSiteManagement>>
) -> impl Responder {
    let object = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    match object.as_str() {
        "site" => {
            let site : GeographicSite = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf674.lock().unwrap().add_site(site).await;
            match result {
                Ok(r) => {
                    //let json = serde_json::to_string(
                    let item = r.first().unwrap().clone();
                    HttpResponse::Created().json(item)
                },
                Err(e) => HttpResponse::BadRequest().json(e),
            }
        },
        _ => HttpResponse::BadRequest().json(PlatypusError::from("TMF674: Invalid Object"))
    }
}

/// Get a list
#[get("/tmf-api/geographicSiteManagement/v4/{object}")]
pub async fn tmf674_list_handler(
    path : web::Path<String>,
    tmf674: web::Data<Mutex<TMF674GeographicSiteManagement>>,
    query : web::Query<QueryOptions>,
) -> impl Responder {
    let object = path.into_inner();
    let query_opts = query.into_inner();

    match object.as_str() {
        "site" => {
            let output = tmf674.lock().unwrap().get_sites(query_opts).await;
            match output {
                Ok(o) => HttpResponse::Ok().json(o),
                Err(e) => HttpResponse::InternalServerError().json(e),
            }
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("TMF674: Invalid Object"))
    }
}

/// Get a specific object
#[get("/tmf-api/geographicSiteManagement/v4/{object}/{id}")]
pub async fn tmf674_get_handler(
    path : web::Path<(String,String)>,
    _tmf674: web::Data<Mutex<TMF674GeographicSiteManagement>>,
    query : web::Query<QueryOptions>,
) -> impl Responder {
    let (object,_id) = path.into_inner();
    let _query_opts = query.into_inner();
    
    match object.as_str() {
        _ => HttpResponse::BadRequest().json(PlatypusError::from("TMF674: Invalid Object"))
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    env_logger::init();

    info!("Starting {pkg} v{ver}");

    let persist = Persistence::new().await;

    let tmf620 = TMF620CatalogManagement::new(persist.clone());
    let tmf632 = TMF632PartyManagement::new(persist.clone());
    let tmf674 = TMF674GeographicSiteManagement::new(persist.clone());

    let config = Config::new();

    // Extract port crom config, default if not found
    let port = config.get("PLATYPUS_PORT").unwrap_or("8000".to_string());
    let port = port.parse::<u16>().unwrap();
   
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(tmf620.clone())))
            .app_data(web::Data::new(Mutex::new(tmf632.clone())))
            .app_data(web::Data::new(Mutex::new(tmf674.clone())))
            .app_data(web::Data::new(Mutex::new(config.clone())))
            .service(tmf620_post_handler)
            .service(tmf620_list_handler)
            .service(tmf620_get_handler)
            .service(tmf620_patch_handler)
            .service(tmf620_delete_handler)
            .service(tmf674_post_handler)
            .service(tmf674_list_handler)
            .service(tmf674_get_handler)
            .service(tmf632_post_handler)
            .service(tmf632_list_handler)
            .service(tmf632_get_handler)
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0",port))?
        .run()
        .await
}
