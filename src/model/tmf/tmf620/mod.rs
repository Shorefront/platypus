//! TMF620 Module
//! 

use std::sync::Mutex;
use tmf620_catalog_management::TMF620CatalogManagement;
use actix_web::{get,patch,post,delete,web, HttpResponse, Responder};
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::category::Category;
use tmflib::tmf620::product_specification::ProductSpecification;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::product_offering_price::ProductOfferingPrice;
use tmflib::{
    HasId,
    HasLastUpdate
};
use log::error;

use crate::common::error::PlatypusError;
use crate::common::persist::Persistence;
use crate::QueryOptions;


pub mod tmf620_catalog_management;

// Place actix_web config functions here

use crate::model::tmf::{
    render_list_output,
    render_get_output,
    render_post_output,
    render_patch_output
};

/// Get a list
#[get("/tmf-api/productCatalogManagement/v4/{object}")]
pub async fn tmf620_list_handler(
    path : web::Path<String>,
    tmf620: web::Data<Mutex<TMF620CatalogManagement>>,
    persist: web::Data<Mutex<Persistence>>,
    query : web::Query<QueryOptions>,
) -> impl Responder {
    let object = path.into_inner();
    let query_opts = query.into_inner();
    let persist = persist.lock().unwrap();
    // Now have to pass persistence into tmf module here
    let mut tmf620 = tmf620.lock().unwrap();
    tmf620.persist(persist.clone());

    match object.as_str() {
        "catalog" => {
            let output = tmf620.get_catalogs(query_opts).await;
            render_list_output(output)
        },
        "category" => {
            let output = tmf620.get_categories(query_opts).await;
            render_list_output(output)
        },
        "productSpecification" => {
            let output = tmf620.get_specifications(query_opts).await;
            render_list_output(output)
        },
        "productOffering" => {
            let output = tmf620.get_offers(query_opts).await;
            render_list_output(output)
        }
        "productOfferingPrice" => {
            let output = tmf620.get_prices(query_opts).await;
            render_list_output(output)   
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
    persist: web::Data<Mutex<Persistence>>,
    query : web::Query<QueryOptions>,
) -> impl Responder {
    let (object,id) = path.into_inner();
    let query_opts = query.into_inner();
    let persist = persist.lock().unwrap();
    let mut tmf620 = tmf620.lock().unwrap();
    tmf620.persist(persist.clone());
    match object.as_str() {
        "catalog" => {
            let output = tmf620.get_catalog(id,query_opts).await;
            render_get_output(output)
        },
        "category" => {
            let output = tmf620.get_category(id,query_opts).await;
            render_get_output(output)
        },
        "productSpecification" => {
            let output = tmf620.get_specification(id,query_opts).await;
            render_get_output(output)
        },
        "productOffering" => {
            let output = tmf620.get_offer(id,query_opts).await;
            render_get_output(output)
        },
        "productOfferingPrice" => {
            let output = tmf620.get_price(id,query_opts).await;
            render_get_output(output)
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

/// Update an object
#[patch("/tmf-api/productCatalogManagement/v4/{object}/{id}")]
pub async fn tmf620_patch_handler(
    path : web::Path<(String,String)>,
    tmf620: web::Data<Mutex<TMF620CatalogManagement>>,
    persist: web::Data<Mutex<Persistence>>,
    raw: web::Bytes,
) -> impl Responder {
    let (object,id) = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf620 = tmf620.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf620.persist(persist.clone());
    match object.as_str() {
        "category" => {
            let category : Category = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf620.patch_category(id, category).await;
            render_patch_output(result)
        },
        "catalog" => {
            let catalog : Catalog = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf620.patch_catalog(id, catalog).await;
            render_patch_output(result)
        },
        "productSpecification" => {
            let specification : ProductSpecification = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf620.patch_specification(id, specification).await;
            render_patch_output(result)
        },
        "productOffering" => {
            let offering : ProductOffering = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf620.patch_offering(id, offering).await;
            render_patch_output(result)
        },
        "productOfferingPrice"  => {
            let price : ProductOfferingPrice = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf620.patch_price(id, price).await;
            render_patch_output(result)
        },
        _ => HttpResponse::BadRequest().json(PlatypusError::from("PATCH: Bad object: {object}"))
    } 
}


/// Create an object
#[post("/tmf-api/productCatalogManagement/v4/{object}")]
pub async fn tmf620_post_handler(
    path : web::Path<String>,
    raw: web::Bytes,
    tmf620: web::Data<Mutex<TMF620CatalogManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let object = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf620 = tmf620.lock().unwrap();
    let persist = persist.lock().unwrap();
    // Set persistance into TMF object
    tmf620.persist(persist.clone());
    match object.as_str() {
        // Create specification 
        "category" => {
            let category : Category = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf620.add_category(category).await;
            render_post_output(result)
        },
        "catalog" => {
            let catalog : Catalog = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf620.add_catalog(catalog).await;
            match result {
                Ok(r) => {
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
            let result = tmf620.add_specification(specification).await;
            match result {
                Ok(r) => {
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
            let result = tmf620.add_offering(offering).await;
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
            let result = tmf620.add_price(price).await;
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

/// Delete an object
#[delete("/tmf-api/productCatalogManagement/v4/{object}/{id}")]
pub async fn tmf620_delete_handler(
    path : web::Path<(String,String)>,
    tmf620: web::Data<Mutex<TMF620CatalogManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let (object,id) = path.into_inner();
    let mut tmf620 = tmf620.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf620.persist(persist.clone());
    match object.as_str() {
        "catalog" => {
            match tmf620.delete_catalog(id).await {
                Ok(_b) => HttpResponse::NoContent(),
                Err(e) => {
                    error!("Could not delete: {e}");
                    HttpResponse::BadRequest()
                },     
            }    
        },
        "category" => {
            match tmf620.delete_category(id).await {
                Ok(_b) => HttpResponse::NoContent(),
                Err(e) => {
                    error!("Could not delete: {e}");
                    HttpResponse::BadRequest()
                },     
            }    
        },
        "productSpecification" => {
            match tmf620.delete_specification(id).await {
                Ok(_b) => HttpResponse::NoContent(),
                Err(e) => {
                    error!("Could not delete: {e}");
                    HttpResponse::BadRequest()
                },
            }
        },
        "productOffering" => {
            match tmf620.delete_offering(id).await {
                Ok(_b) => HttpResponse::NoContent(),
                Err(e) => {
                    error!("Could not delete: {e}");
                    HttpResponse::BadRequest()
                },
            }
        },
        "productOfferingPrice"  => {
            match tmf620.delete_price(id).await {
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

pub fn config_tmf620(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf620 = TMF620CatalogManagement::new(None);
    cfg
        .app_data(web::Data::new(Mutex::new(tmf620.clone())))
        .service(tmf620_list_handler)
        .service(tmf620_get_handler)
        .service(tmf620_patch_handler)
        .service(tmf620_post_handler)
        .service(tmf620_delete_handler);
}