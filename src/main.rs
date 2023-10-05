
use log::info;

mod model;
mod template;
mod common;

use actix_web::middleware::Logger;
use actix_web::{get,post,web,App, HttpResponse,HttpServer, Responder};

use log::error;

use std::sync::Mutex;

// SurrealDB
use serde::Deserialize;
use surrealdb::engine::local::Mem;
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

// TMFLIB
use common::config::Config;
use common::error::PlatypusError;
//use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::category::Category;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf629::customer::Customer;
use tmflib::tmf629::customer::CUST_STATUS;
use tmflib::tmf648::quote::Quote;

use crate::template::component::ComponentTemplate;
//use crate::template::product::ProductTemplate;
//use crate::model::component::product::ProductComponent;
use crate::template::product::ProductTemplate;
use crate::model::tmf::tmf620_catalog_management::TMF620CatalogManagement;

#[derive(Debug,Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Option<Thing>,
}

#[post("/compose/template/product")]
pub async fn template_product_handler(
    body : web::Json<ProductTemplate>
) -> impl Responder {
    let data = body.into_inner();

    HttpResponse::Ok().json(data)
}

#[post("/compose/template/component")]
pub async fn template_component_handler(
    body : web::Json<ComponentTemplate>
) -> impl Responder {
    let data = body.into_inner();

    HttpResponse::Ok().json(data)
}

#[post("/tmflib/tmf620/offer")]
pub async fn tmf620_handler(
    body : web::Json<ProductOffering>
) -> impl Responder {
    let data = body.into_inner();

    // Since this is a create, we need to generate the Id / Href
    let new_offer = ProductOffering::from(data);
    HttpResponse::Ok().json(new_offer)
}

#[get("/tmflib/tmf620/category")]
pub async fn tmf620_category_list(
    tmf620: web::Data<Mutex<TMF620CatalogManagement>>
) -> impl Responder {
    match tmf620.lock().expect("Could not lock DB").get_categories().await {
        Ok(r) => {
            
            HttpResponse::Ok().json(r.clone())
        },
        Err(e) => {
            error!("Error: {e}");
            let msg = PlatypusError {
                message : e.to_string(),
            };
            HttpResponse::BadRequest().json(msg)
        },  
    }
}

#[post("/tmflib/tmf620/category")]
pub async fn tmf620_category_create(
    body : web::Json<Category>,
    tmf620: web::Data<Mutex<TMF620CatalogManagement>>,
) -> impl Responder {
    //let tmf620 = tmf620.into_inner();
    let mut data = body.into_inner(); 
    // Need to generate new id / href as we're creating
    data.generate_id();
    match tmf620.lock().expect("Could not lock db").add_category(data.clone()).await {
        Ok(r) => {
            
            HttpResponse::Ok().json(data.clone())
        },
        Err(e) => {
            error!("Error: {e}");
            let msg = PlatypusError {
                message : e.to_string(),
            };
            HttpResponse::BadRequest().json(msg)
        },
    }
    
}


#[post("/tmflib/tmf629/customer")]
pub async fn tmf629_create_handler(
    body : web::Json<Customer>,
    _db   : web::Data<Surreal<Db>>
) -> impl Responder {
    let mut data = body.into_inner();
    data.generate_code();
    // Since this a new customer we have to regenerate the href
    data.generate_href();
    data.status = Some(CUST_STATUS.to_string());
    HttpResponse::Ok().json(data)
}

#[get("/tmflib/tmf629/customer/{id}")]
pub async fn tmf629_get_handler(

) -> impl Responder {
    HttpResponse::Ok()
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

    let db = Surreal::new::<Mem>(()).await.expect("Could not create DB");

    db.use_ns("tmflib").use_db("composable").await.expect("Could not set DB NS");

    let tmf620 = TMF620CatalogManagement::new(db.clone());

    info!("Starting {pkg} v{ver}");

    let _cfg = Config::new();
   
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(tmf620.clone())))
            .service(tmf620_handler)
            .service(tmf620_category_create)
            .service(tmf620_category_list)
            .service(tmf629_create_handler)
            .service(tmf648_create_handler)
            .service(template_component_handler)
            .service(template_product_handler)
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0",8000))?
        .run()
        .await
}
