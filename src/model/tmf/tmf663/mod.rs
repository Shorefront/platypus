//! Shopping Cart Management

use crate::common::error::PlatypusError;
use crate::common::persist::Persistence;
use crate::QueryOptions;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use std::sync::Mutex;
use tmf663_shopping_cart::TMF663ShoppingCartManagement;
use tmflib::tmf663::shopping_cart::ShoppingCart;

use crate::model::tmf::{
    render_delete_output, render_get_output, render_list_output, render_patch_output,
    render_post_output,
};

pub mod tmf663_shopping_cart;

/// Get a list
#[get("/tmf-api/shoppingCart/v4/{object}")]
pub async fn tmf663_list_handler(
    path: web::Path<String>,
    query: web::Query<QueryOptions>,
    tmf663: web::Data<Mutex<TMF663ShoppingCartManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let object = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf663 = tmf663.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf663.persist(persist.clone());
    match object.as_str() {
        "shoppingCart" => {
            let sites = tmf663.get_carts(query_opts).await;
            render_list_output(sites)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object")),
    }
}

#[get("/tmf-api/shoppingCart/v4/{object}/{id}")]
pub async fn tmf663_get_handler(
    path: web::Path<(String, String)>,
    query: web::Query<QueryOptions>,
    tmf663: web::Data<Mutex<TMF663ShoppingCartManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let (object, id) = path.into_inner();
    let query_opts = query.into_inner();
    let mut tmf663 = tmf663.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf663.persist(persist.clone());
    match object.as_str() {
        "shoppingCart" => {
            let customers = tmf663.get_cart(id, query_opts).await;
            render_get_output(customers)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object")),
    }
}

/// Create an object
#[post("/tmf-api/shoppingCart/v4/{object}")]
pub async fn tmf663_post_handler(
    path: web::Path<String>,
    raw: web::Bytes,
    tmf663: web::Data<Mutex<TMF663ShoppingCartManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let object = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf663 = tmf663.lock().unwrap();
    let persist = persist.lock().unwrap();
    // Set persistance into TMF object
    tmf663.persist(persist.clone());
    match object.as_str() {
        "shoppingCart" => {
            let cart: ShoppingCart =
                serde_json::from_str(json.as_str()).expect("Could not parse TMF663 objet");
            let result = tmf663.add_cart(cart).await;
            render_post_output(result)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object: {object}")),
    }
}

/// Update an object
#[patch("/tmf-api/shoppingCart/v4/{object}/{id}")]
pub async fn tmf663_patch_handler(
    path: web::Path<(String, String)>,
    tmf663: web::Data<Mutex<TMF663ShoppingCartManagement>>,
    persist: web::Data<Mutex<Persistence>>,
    raw: web::Bytes,
) -> impl Responder {
    let (object, id) = path.into_inner();
    let json = String::from_utf8(raw.to_vec()).unwrap();
    let mut tmf663 = tmf663.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf663.persist(persist.clone());
    match object.as_str() {
        "shoppingCart" => {
            let site: ShoppingCart = serde_json::from_str(json.as_str()).unwrap();
            let result = tmf663.update_cart(id, site).await;
            render_patch_output(result)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("PATCH: Bad object: {object}")),
    }
}

#[delete("/tmf-api/shoppingCart/v4/{object}/{id}")]
pub async fn tmf663_delete_handler(
    path: web::Path<(String, String)>,
    tmf663: web::Data<Mutex<TMF663ShoppingCartManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let (object, id) = path.into_inner();
    let mut tmf663 = tmf663.lock().unwrap();
    let persist = persist.lock().unwrap();
    tmf663.persist(persist.clone());
    match object.as_str() {
        "shoppingCart" => {
            let cart = tmf663.delete_cart(id).await;
            render_delete_output(cart)
        }
        _ => HttpResponse::BadRequest().json(PlatypusError::from("Invalid Object")),
    }
}

pub fn config_tmf663(cfg: &mut web::ServiceConfig) {
    // Place our configuration into cfg
    // NB: Since we are adding via this method, we don't have access to persist class
    // so we need to get access to that via web_data instead now.
    let tmf663 = TMF663ShoppingCartManagement::new(None);
    cfg.service(tmf663_list_handler)
        .service(tmf663_get_handler)
        .service(tmf663_post_handler)
        .service(tmf663_patch_handler)
        .service(tmf663_delete_handler)
        .app_data(web::Data::new(Mutex::new(tmf663.clone())));
}
