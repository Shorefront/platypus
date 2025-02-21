//! Hub module for end-point registration

use actix_web::{post,delete,web, HttpResponse, Responder};
use super::error::PlatypusError;
use crate::common::persist::Persistence;
use std::sync::Mutex;
use log::error;
use serde::{Deserialize,Serialize};
use tmflib::Uri;


#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct NotificationEndpoint {
    id: String,
    domain : String,
    filter : Option<String>,
    callback: Uri,
    query: Option<String>,
}

#[derive(Clone, Debug)]
pub struct HubManagement {
    persist : Option<Persistence>,
}

impl HubManagement {
    pub fn new(persist : Option<Persistence>) -> Self {
        HubManagement {
            persist,
        }
    }
    pub fn persist(&mut self, persist : Persistence) {
        self.persist = Some(persist);
    }

    pub async fn register_hub(&mut self, hub : NotificationEndpoint) -> Result<NotificationEndpoint,PlatypusError> {
        self.persist.as_mut().unwrap().create_hub_item(hub.clone()).await
    }   
}

pub fn render_register_hub<T : Serialize>(output : Result<T,PlatypusError>) -> impl Responder {
    match output {
        Ok(b) => {
            HttpResponse::Created().json(b)
        }
        Err(e) => {
            error!("Could not delete: {e}");
            HttpResponse::Conflict().json(e)
        },     
    }
}

pub fn render_delete_hub<T : Serialize>(output : Result<T,PlatypusError>) -> impl Responder {
    match output {
        Ok(_b) => HttpResponse::NoContent().finish(),
        Err(e) => {
            error!("Could not delete: {e}");
            HttpResponse::NotFound().json(e)
        },     
    }     
}

#[post("/tmf-api/hub")]
pub async fn hub_handle_post(
    raw: web::Bytes,
    hub: web::Data<Mutex<HubManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let mut hub = hub.lock().unwrap();
    hub.persist(persist.lock().unwrap().clone());

    let response = hub.register_hub(hub).await;
    render_register_hub(response)
}

#[delete("/tmf-api/hub/{hub_id}")]
pub async fn hub_handle_delete(
    path : web::Path<String>,
    hub: web::Data<Mutex<HubManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let id = path.into_inner();
    let mut hub = hub.lock().unwrap();
    hub.persist(persist.lock().unwrap().clone());
    HttpResponse::Ok().body("Delete hub!")
}

pub fn config_hub(cfg: &mut web::ServiceConfig) {
    let hub_mgt = HubManagement::new(None);
    cfg
        .service(hub_handle_post)
        .service(hub_handle_delete)
        .app_data(web::Data::new(Mutex::new(hub_mgt.clone())));
}