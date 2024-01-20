//! TMF620 Module
//! 
use actix_web::web;
use std::sync::Mutex;
use tmf620_catalog_management::TMF620CatalogManagement;

pub mod tmf620_catalog_management;

// Place actix_web config functions here

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
        }
        _ => {
            HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object: {object}"))
        }
    }
}

pub fn config_tmf620(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    let tmf620 = TMF620CatalogManagement::new(None);
    cfg.app_data(web::Data::new(Mutex::new(tmf620.clone())));
    cfg.service(tmf620_post_handler);
}