
use log::info;

mod model;
mod template;
mod common;

use actix_web::middleware::Logger;
use actix_web::{post,web,App, HttpResponse,HttpServer, Responder};

use common::config::Config;
use tmflib::tmf620::product_specification::{ProductSpecification, ProductSpecificationCharacteristic};
use tmflib::tmf620::tmf620_catalog_management::TMF620CatalogueManagement;
use tmflib::tmf648::quote::Quote;

use crate::template::component::ComponentTemplate;
//use crate::template::product::ProductTemplate;
//use crate::model::component::product::ProductComponent;
use crate::template::product::ProductTemplate;

#[post("/tmflib/tmf620/")]
pub async fn tmf620_handler(

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

    info!("Starting {pkg} v{ver}");

    let _cfg = Config::new();
   
    HttpServer::new(move || {
        App::new()
            .service(tmf620_handler)
            .service(tmf648_create_handler)
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0",8000))?
        .run()
        .await
}
